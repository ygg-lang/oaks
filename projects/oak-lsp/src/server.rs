use crate::{
    service::LanguageService,
    types::{InitializeParams, Range},
};
use oak_vfs::{LineMap, Vfs, WritableVfs};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};

pub struct LspServer<S: LanguageService> {
    service: Arc<S>,
}

impl<S: LanguageService + 'static> LspServer<S> {
    pub fn new(service: Arc<S>) -> Self {
        Self { service }
    }

    pub async fn run<R, W>(&self, reader: R, mut writer: W) -> anyhow::Result<()>
    where
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        let mut reader = BufReader::new(reader);

        loop {
            let mut content_length = 0;

            // Read headers
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).await? == 0 {
                    return Ok(());
                }
                if line.trim().is_empty() {
                    break;
                }
                if line.to_lowercase().starts_with("content-length:") {
                    content_length = line["content-length:".len()..].trim().parse::<usize>()?;
                }
            }

            if content_length == 0 {
                continue;
            }

            // Read body
            let mut body = vec![0u8; content_length];
            reader.read_exact(&mut body).await?;
            let body_str = String::from_utf8(body)?;
            let request: Value = serde_json::from_str(&body_str)?;

            // Handle request
            if let Some(id) = request.get("id") {
                // Method call
                let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
                let params = request.get("params").cloned().unwrap_or(json!({}));

                let response = self.handle_request(id.clone(), method, params).await;
                self.send_payload(&mut writer, response).await?;
            }
            else {
                // Notification
                let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
                let params = request.get("params").cloned().unwrap_or(json!({}));
                self.handle_notification(method, params, &mut writer).await?;
            }
        }
    }

    async fn handle_request(&self, id: Value, method: &str, params: Value) -> Value {
        match method {
            "initialize" => {
                let params: InitializeParams = serde_json::from_value(params).unwrap_or_default();
                self.service.initialize(params).await;
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "capabilities": {
                            "textDocumentSync": 1,
                            "hoverProvider": true,
                            "completionProvider": {
                                "resolveProvider": false,
                                "triggerCharacters": [".", "<", "@", ":"]
                            },
                            "definitionProvider": true,
                            "referencesProvider": true,
                            "documentSymbolProvider": true,
                            "workspaceSymbolProvider": true,
                            "renameProvider": true
                        }
                    }
                })
            }
            "shutdown" => {
                self.service.shutdown().await;
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": null
                })
            }
            "textDocument/hover" => {
                if let (Some(uri), Some(pos)) = (params.get("textDocument").and_then(|d| d.get("uri")).and_then(|u| u.as_str()), params.get("position")) {
                    if let Some(source) = self.service.vfs().get_source(uri) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as u32;
                        let character = pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
                        let line_map = LineMap::from_source(&source);
                        let offset = line_map.line_col_utf16_to_offset(&source, line, character);

                        if let Some(hover) = self.service.hover(uri, Range { start: offset, end: offset }).await {
                            return json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "contents": { "kind": "markdown", "value": hover.contents }
                                }
                            });
                        }
                    }
                }
                json!({ "jsonrpc": "2.0", "id": id, "result": null })
            }
            "textDocument/completion" => {
                if let (Some(uri), Some(pos)) = (params.get("textDocument").and_then(|d| d.get("uri")).and_then(|u| u.as_str()), params.get("position")) {
                    if let Some(source) = self.service.vfs().get_source(uri) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as u32;
                        let character = pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
                        let line_map = LineMap::from_source(&source);
                        let offset = line_map.line_col_utf16_to_offset(&source, line, character);

                        let items = self.service.completion(uri, offset).await;
                        return json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": items
                        });
                    }
                }
                json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            "textDocument/definition" => {
                if let (Some(uri), Some(pos)) = (params.get("textDocument").and_then(|d| d.get("uri")).and_then(|u| u.as_str()), params.get("position")) {
                    if let Some(source) = self.service.vfs().get_source(uri) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as u32;
                        let character = pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
                        let line_map = LineMap::from_source(&source);
                        let offset = line_map.line_col_utf16_to_offset(&source, line, character);

                        let locations = self.service.definition(uri, Range { start: offset, end: offset }).await;
                        let mut result = Vec::new();
                        for loc in locations {
                            if let Some(target_source) = self.service.vfs().get_source(&loc.uri) {
                                let target_line_map = LineMap::from_source(&target_source);
                                let (start_line, start_character) = target_line_map.offset_to_line_col_utf16(&target_source, loc.range.start);
                                let (end_line, end_character) = target_line_map.offset_to_line_col_utf16(&target_source, loc.range.end);
                                result.push(json!({
                                    "uri": loc.uri,
                                    "range": {
                                        "start": { "line": start_line, "character": start_character },
                                        "end": { "line": end_line, "character": end_character }
                                    }
                                }));
                            }
                        }
                        return json!({ "jsonrpc": "2.0", "id": id, "result": result });
                    }
                }
                json!({ "jsonrpc": "2.0", "id": id, "result": null })
            }
            _ => {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": {
                        "code": -32601,
                        "message": format!("Method not found: {}", method)
                    }
                })
            }
        }
    }

    async fn handle_notification<W: AsyncWrite + Unpin>(&self, method: &str, params: Value, writer: &mut W) -> anyhow::Result<()> {
        match method {
            "initialized" => {
                self.service.initialized().await;
            }
            "exit" => {
                std::process::exit(0);
            }
            "textDocument/didOpen" => {
                if let Some(doc) = params.get("textDocument") {
                    if let (Some(uri), Some(text)) = (doc.get("uri").and_then(|u| u.as_str()), doc.get("text").and_then(|t| t.as_str())) {
                        self.service.vfs().write_file(uri, text.to_string());
                        self.publish_diagnostics(uri, writer).await?;
                    }
                }
            }
            "textDocument/didChange" => {
                if let (Some(uri), Some(changes)) = (params.get("textDocument").and_then(|d| d.get("uri")).and_then(|u| u.as_str()), params.get("contentChanges").and_then(|c| c.as_array())) {
                    if let Some(change) = changes.first() {
                        if let Some(text) = change.get("text").and_then(|t| t.as_str()) {
                            self.service.vfs().write_file(uri, text.to_string());
                            self.publish_diagnostics(uri, writer).await?;
                        }
                    }
                }
            }
            "textDocument/didSave" => {
                if let Some(uri) = params.get("textDocument").and_then(|d| d.get("uri")).and_then(|u| u.as_str()) {
                    self.service.did_save(uri).await;
                }
            }
            "textDocument/didClose" => {
                if let Some(uri) = params.get("textDocument").and_then(|d| d.get("uri")).and_then(|u| u.as_str()) {
                    self.service.did_close(uri).await;
                }
            }
            _ => {}
        }
        Ok(())
    }

    async fn publish_diagnostics<W: AsyncWrite + Unpin>(&self, uri: &str, writer: &mut W) -> anyhow::Result<()> {
        use oak_vfs::LineMap;
        let diags = self.service.diagnostics(uri).await;
        if let Some(source) = self.service.vfs().get_source(uri) {
            let line_map = LineMap::from_source(&source);
            let mut result = Vec::new();
            for diag in diags {
                let (start_line, start_character) = line_map.offset_to_line_col_utf16(&source, diag.range.start);
                let (end_line, end_character) = line_map.offset_to_line_col_utf16(&source, diag.range.end);
                result.push(json!({
                    "range": {
                        "start": { "line": start_line, "character": start_character },
                        "end": { "line": end_line, "character": end_character }
                    },
                    "severity": diag.severity.map(|s| s as u32).unwrap_or(1),
                    "message": diag.message,
                    "source": diag.source
                }));
            }
            let payload = json!({
                "jsonrpc": "2.0",
                "method": "textDocument/publishDiagnostics",
                "params": {
                    "uri": uri,
                    "diagnostics": result
                }
            });
            self.send_payload(writer, payload).await?;
        }
        Ok(())
    }

    async fn send_payload<W: AsyncWrite + Unpin>(&self, writer: &mut W, payload: Value) -> anyhow::Result<()> {
        let body = serde_json::to_string(&payload)?;
        let header = format!("Content-Length: {}\r\n\r\n", body.len());
        writer.write_all(header.as_bytes()).await?;
        writer.write_all(body.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }
}

/// Creates an Axum router for the language server.
#[cfg(feature = "axum")]
pub fn axum_router<S: LanguageService + 'static>(service: Arc<S>) -> axum::Router {
    use axum::{Json, Router, extract::State, routing::post};

    Router::new()
        .route(
            "/lsp",
            post(|State(service): State<Arc<S>>, Json(request): Json<Value>| async move {
                let server = LspServer::new(service);
                if let Some(id) = request.get("id") {
                    let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
                    let params = request.get("params").cloned().unwrap_or(json!({}));
                    let response = server.handle_request(id.clone(), method, params).await;
                    Json(response)
                }
                else {
                    // Notifications in HTTP are tricky, just return null
                    Json(json!(null))
                }
            }),
        )
        .with_state(service)
}
