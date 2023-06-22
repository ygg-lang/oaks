#![feature(new_range_api)]
use core::range::Range;
use oak_lsp::service::LanguageService;
pub use oak_semantic_search::{NoSemanticSearch, SemanticSearch};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::info;

pub mod handlers;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
}

/// A generic MCP server that wraps an Oak language service.
pub struct McpServer<S: LanguageService, E: SemanticSearch = NoSemanticSearch> {
    service: Arc<S>,
    searcher: Option<Arc<E>>,
}

impl<S: LanguageService + 'static> McpServer<S, NoSemanticSearch>
where
    S::Vfs: oak_vfs::WritableVfs,
{
    pub fn new(service: S) -> Self {
        Self { service: Arc::new(service), searcher: None }
    }
}

impl<S: LanguageService + 'static, E: SemanticSearch + 'static> McpServer<S, E>
where
    S::Vfs: oak_vfs::WritableVfs,
{
    pub fn with_searcher<NewE: SemanticSearch>(self, searcher: NewE) -> McpServer<S, NewE> {
        McpServer { service: self.service, searcher: Some(Arc::new(searcher)) }
    }

    pub async fn run(&self) -> tokio::io::Result<()> {
        let stdin = tokio::io::stdin();
        let mut stdout = tokio::io::stdout();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            let input = line.trim();
            if input.is_empty() {
                line.clear();
                continue;
            }

            if let Ok(request) = serde_json::from_str::<JsonRpcRequest>(input) {
                let response = self.handle_request(request).await;
                let response_json = serde_json::to_string(&response).unwrap();
                stdout.write_all(response_json.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
            }
            else if let Ok(notification) = serde_json::from_str::<JsonRpcNotification>(input) {
                self.handle_notification(notification).await;
            }

            line.clear();
        }

        Ok(())
    }

    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {
                            "listChanged": false
                        }
                    },
                    "serverInfo": {
                        "name": "oak-mcp",
                        "version": "0.0.1"
                    }
                })),
                error: None,
            },
            "tools/list" => {
                let tools: Value = serde_json::from_str(include_str!("tools.json")).unwrap();
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!({
                        "tools": tools
                    })),
                    error: None,
                }
            }
            "tools/call" => {
                let params = request.params.unwrap_or(Value::Null);
                let name = params.get("name").and_then(|v| v.as_str()).unwrap_or_default();
                let args = params.get("arguments").cloned().unwrap_or(Value::Null);

                match self.handle_tool_call(name, args).await {
                    Ok(result) => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: Some(json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": serde_json::to_string_pretty(&result).unwrap()
                                }
                            ]
                        })),
                        error: None,
                    },
                    Err(e) => JsonRpcResponse { jsonrpc: "2.0".to_string(), id: request.id, result: None, error: Some(JsonRpcError { code: -32000, message: e, data: None }) },
                }
            }
            _ => JsonRpcResponse { jsonrpc: "2.0".to_string(), id: request.id, result: None, error: Some(JsonRpcError { code: -32601, message: format!("Method not found: {}", request.method), data: None }) },
        }
    }

    async fn handle_tool_call(&self, name: &str, args: Value) -> Result<Value, String> {
        match name {
            "hover" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let offset = args.get("offset").and_then(|v| v.as_u64()).ok_or("Missing offset")? as usize;

                let hover = self.service.hover(uri, Range { start: offset, end: offset }).await;
                Ok(json!(hover))
            }
            "symbols" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let symbols = self.service.document_symbols(uri).await;
                Ok(json!(symbols))
            }
            "definition" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let offset = args.get("offset").and_then(|v| v.as_u64()).ok_or("Missing offset")? as usize;

                let locs = self.service.definition(uri, Range { start: offset, end: offset }).await;
                Ok(json!(locs))
            }
            "references" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let offset = args.get("offset").and_then(|v| v.as_u64()).ok_or("Missing offset")? as usize;
                let locs = self.service.references(uri, Range { start: offset, end: offset }).await;
                Ok(json!(locs))
            }
            "diagnostics" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let diagnostics = self.service.diagnostics(uri).await;
                Ok(json!(diagnostics))
            }
            "completion" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let offset = args.get("offset").and_then(|v| v.as_u64()).ok_or("Missing offset")? as usize;
                let items = self.service.completion(uri, offset).await;
                Ok(json!(items))
            }
            "search" => {
                let query = args.get("query").and_then(|v| v.as_str()).ok_or("Missing query")?;
                let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(5) as usize;

                if let Some(searcher) = &self.searcher {
                    let results = searcher.search(query, limit).await.map_err(|e| e.to_string())?;
                    Ok(json!(results))
                }
                else {
                    Err("Semantic search is not enabled on this server".to_string())
                }
            }
            "set_file_content" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let content = args.get("content").and_then(|v| v.as_str()).ok_or("Missing content")?;

                use oak_vfs::WritableVfs;
                let vfs = self.service.vfs();
                vfs.write_file(uri, content.to_string());
                Ok(json!({"status": "ok"}))
            }
            "semantic_search" => {
                let query = args.get("query").and_then(|v| v.as_str()).ok_or("Missing query")?;
                let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(5) as usize;

                if let Some(searcher) = &self.searcher {
                    let results = searcher.search(query, limit).await.map_err(|e| e.to_string())?;
                    Ok(json!(results))
                }
                else {
                    Err("Semantic search is not enabled on this server".to_string())
                }
            }
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }

    pub async fn handle_notification(&self, notification: JsonRpcNotification) {
        info!("Received notification: {}", notification.method);
    }
}

/// Extension trait for language services to provide MCP integration.
pub trait OakMcpService: LanguageService + Sized + 'static
where
    Self::Vfs: oak_vfs::WritableVfs,
{
    /// Convert this service into an Oak MCP server.
    fn into_mcp_server(self) -> McpServer<Self> {
        McpServer::new(self)
    }

    /// Create an Axum router for this MCP service.
    #[cfg(feature = "axum")]
    fn into_mcp_axum_router(self) -> axum::Router {
        crate::handlers::axum_handlers::create_router(self)
    }

    /// Register this MCP service with an Actix-web config.
    #[cfg(feature = "actix-web")]
    fn register_mcp_actix(self, cfg: &mut actix_web::web::ServiceConfig) {
        crate::handlers::actix_handlers::config(cfg, self)
    }
}

impl<S: LanguageService + 'static> OakMcpService for S where S::Vfs: oak_vfs::WritableVfs {}
