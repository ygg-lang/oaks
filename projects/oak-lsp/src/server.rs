use crate::{
    service::LanguageService,
    types::{InitializeParams, Range},
};
use oak_vfs::{LineMap, Vfs, WritableVfs};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum LspError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    Json(String),
    Other(String),
}

impl Display for LspError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LspError::Io(e) => write!(f, "IO error: {}", e),
            LspError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            LspError::Json(e) => write!(f, "JSON error: {}", e),
            LspError::Other(s) => write!(f, "{}", s),
        }
    }
}

impl Error for LspError {}

impl From<std::io::Error> for LspError {
    fn from(e: std::io::Error) -> Self {
        LspError::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for LspError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        LspError::Utf8(e)
    }
}

impl From<String> for LspError {
    fn from(e: String) -> Self {
        LspError::Json(e)
    }
}

impl From<anyhow::Error> for LspError {
    fn from(e: anyhow::Error) -> Self {
        LspError::Other(e.to_string())
    }
}

impl From<std::num::ParseIntError> for LspError {
    fn from(e: std::num::ParseIntError) -> Self {
        LspError::Other(e.to_string())
    }
}

impl From<serde_json::Error> for LspError {
    fn from(e: serde_json::Error) -> Self {
        LspError::Json(e.to_string())
    }
}

pub struct LspServer<S: LanguageService> {
    service: Arc<S>,
}

impl<S: LanguageService + 'static> LspServer<S> {
    pub fn new(service: Arc<S>) -> Self {
        Self { service }
    }

    pub async fn run<R, W>(&self, reader: R, mut writer: W) -> Result<(), LspError>
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
            let request: JsonValue = serde_json::from_str(&body_str)?;

            // Handle request
            if let Some(id) = request.get("id") {
                // Method call
                let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
                let params = request.get("params").cloned().unwrap_or_else(|| serde_json::json!({}));

                let response = self.handle_request(id.clone(), method, params).await;
                self.send_payload(&mut writer, response).await?;
            }
            else {
                // Notification
                let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");
                let params = request.get("params").cloned().unwrap_or_else(|| serde_json::json!({}));
                self.handle_notification(method, params, &mut writer).await?;
            }
        }
    }

    async fn handle_request(&self, id: JsonValue, method: &str, params: JsonValue) -> JsonValue {
        match method {
            "initialize" => {
                let params: InitializeParams = serde_json::from_value(params).unwrap_or_default();
                self.service.initialize(params).await;
                serde_json::json!({
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
                            "renameProvider": true,
                            "semanticTokensProvider": {
                                "full": true,
                                "range": false,
                                "legend": {
                                    "tokenTypes": [
                                        "class", "parameter", "variable", "function", "keyword", "string", "number", "operator",
                                        "namespace", "struct", "enum", "interface", "type", "decorator", "macro", "comment"
                                    ],
                                    "tokenModifiers": ["declaration", "definition", "readonly", "static", "deprecated", "abstract", "async", "modification", "documentation", "defaultLibrary"]
                                }
                            },
                            "selectionRangeProvider": true,
                            "signatureHelpProvider": {
                                "triggerCharacters": ["(", ","]
                            },
                            "inlayHintProvider": true,
                            "documentFormattingProvider": true,
                            "codeActionProvider": true,
                            "foldingRangeProvider": true
                        }
                    }
                })
            }
            "shutdown" => {
                self.service.shutdown().await;
                serde_json::json!({
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
                            return serde_json::json!({
                                "jsonrpc": "2.0",
                                "id": id,
                                "result": {
                                    "contents": { "kind": "markdown", "value": hover.contents }
                                }
                            });
                        }
                    }
                }
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": null })
            }
            "textDocument/completion" => {
                if let (Some(uri), Some(pos)) = (params.get("textDocument").and_then(|d| d.get("uri")).and_then(|u| u.as_str()), params.get("position")) {
                    if let Some(source) = self.service.vfs().get_source(uri) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as u32;
                        let character = pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
                        let line_map = LineMap::from_source(&source);
                        let offset = line_map.line_col_utf16_to_offset(&source, line, character);

                        let _items = self.service.completion(uri, offset).await;
                        // For now we don't convert completions to JsonValue as it's complex without serde
                        return serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": []
                        });
                    }
                }
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            "textDocument/definition" => {
                if let (Some(uri), Some(pos)) = (params.get("textDocument").and_then(|d| d.get("uri")).and_then(|u| u.as_str()), params.get("position")) {
                    if let Some(source) = self.service.vfs().get_source(uri) {
                        let line = pos.get("line").and_then(|l| l.as_u64()).unwrap_or(0) as u32;
                        let character = pos.get("character").and_then(|c| c.as_u64()).unwrap_or(0) as u32;
                        let line_map = LineMap::from_source(&source);
                        let offset = line_map.line_col_utf16_to_offset(&source, line, character);

                        let _locations = self.service.definition(uri, Range { start: offset, end: offset }).await;
                        // Simplified result for now
                        return serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": null });
                    }
                }
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": null })
            }
            "textDocument/references" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            "textDocument/documentSymbol" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            "workspace/symbol" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            "textDocument/rename" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": null })
            }
            "textDocument/foldingRange" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            "textDocument/semanticTokens/full" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": null })
            }
            "textDocument/semanticTokens/range" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": null })
            }
            "textDocument/selectionRange" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            "textDocument/signatureHelp" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": null })
            }
            "textDocument/inlayHint" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            "textDocument/formatting" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            "textDocument/codeAction" => {
                serde_json::json!({ "jsonrpc": "2.0", "id": id, "result": [] })
            }
            _ => {
                serde_json::json!({
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

    async fn handle_notification<W: AsyncWrite + Unpin>(&self, method: &str, params: JsonValue, writer: &mut W) -> Result<(), LspError> {
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
                        self.service.vfs().write_file(uri, text.to_string().into());
                        self.publish_diagnostics(uri, writer).await?;
                    }
                }
            }
            "textDocument/didChange" => {
                if let (Some(uri), Some(changes)) = (params.get("textDocument").and_then(|d| d.get("uri")).and_then(|u| u.as_str()), params.get("contentChanges").and_then(|c| c.as_array())) {
                    if let Some(change) = changes.first() {
                        if let Some(text) = change.get("text").and_then(|t| t.as_str()) {
                            self.service.vfs().write_file(uri, text.to_string().into());
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

    async fn publish_diagnostics<W: AsyncWrite + Unpin>(&self, uri: &str, writer: &mut W) -> Result<(), LspError> {
        use oak_vfs::LineMap;
        let diags = self.service.diagnostics(uri).await;
        if let Some(source) = self.service.vfs().get_source(uri) {
            let line_map = LineMap::from_source(&source);
            let mut result = Vec::new();
            for diag in diags {
                let (start_line, start_character) = line_map.offset_to_line_col_utf16(&source, diag.range.start);
                let (end_line, end_character) = line_map.offset_to_line_col_utf16(&source, diag.range.end);
                result.push(serde_json::json!({
                    "range": {
                        "start": { "line": start_line, "character": start_character },
                        "end": { "line": end_line, "character": end_character }
                    },
                    "severity": diag.severity.map(|s| s as u32).unwrap_or(1),
                    "message": diag.message,
                    "source": diag.source
                }));
            }
            let payload = serde_json::json!({
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

    async fn send_payload<W: AsyncWrite + Unpin>(&self, writer: &mut W, payload: JsonValue) -> Result<(), LspError> {
        let body = payload.to_string();
        let header = format!("Content-Length: {}\r\n\r\n", body.len());
        writer.write_all(header.as_bytes()).await?;
        writer.write_all(body.as_bytes()).await?;
        writer.flush().await?;
        Ok(())
    }
}
