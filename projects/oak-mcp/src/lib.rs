#![feature(new_range_api)]
use core::range::Range;
use oak_lsp::service::LanguageService;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};
use std::sync::Arc;
use tokio::io::AsyncBufReadExt;
#[cfg(feature = "io-std")]
use tokio::io::{AsyncWriteExt, BufReader};

pub use oak_semantic_search::{NoSemanticSearch, SemanticSearch};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: JsonValue,
    pub method: String,
    pub params: Option<JsonValue>,
}

impl JsonRpcRequest {
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| e.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: JsonValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<JsonValue>,
}

impl JsonRpcNotification {
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| e.to_string())
    }
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

    #[cfg(feature = "io-std")]
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

            if let Ok(request) = JsonRpcRequest::from_json(input) {
                let response = self.handle_request(request).await;
                let response_json = response.to_json();
                stdout.write_all(response_json.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
            }
            else if let Ok(notification) = JsonRpcNotification::from_json(input) {
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
                let tools_str = include_str!("tools.json");
                let tools: JsonValue = serde_json::from_str(tools_str).unwrap();
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
                let params = request.params.unwrap_or(json!(null));
                let name = params.get("name").and_then(|v| v.as_str()).unwrap_or_default();
                let args = params.get("arguments").cloned().unwrap_or(json!(null));

                match self.handle_tool_call(name, args).await {
                    Ok(result) => JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: request.id,
                        result: Some(json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": result.to_string()
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

    async fn handle_tool_call(&self, name: &str, args: JsonValue) -> Result<JsonValue, String> {
        match name {
            "hover" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let offset = args.get("offset").and_then(|v| v.as_u64()).ok_or("Missing offset")? as usize;

                let hover = self.service.hover(uri, Range { start: offset, end: offset }).await;
                Ok(json!(hover.map(|h| h.contents).unwrap_or_default()))
            }
            "symbols" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let _symbols = self.service.document_symbols(uri).await;
                // symbols is Vec<DocumentSymbol>, need proper conversion.
                // For now just returning empty array as placeholder if conversion is complex
                Ok(json!([]))
            }
            "definition" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let offset = args.get("offset").and_then(|v| v.as_u64()).ok_or("Missing offset")? as usize;

                let _locs = self.service.definition(uri, Range { start: offset, end: offset }).await;
                Ok(json!([]))
            }
            "references" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let offset = args.get("offset").and_then(|v| v.as_u64()).ok_or("Missing offset")? as usize;

                let _locs = self.service.references(uri, Range { start: offset, end: offset }).await;
                Ok(json!([]))
            }
            "diagnostics" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let _diagnostics = self.service.diagnostics(uri).await;
                Ok(json!([]))
            }
            "completion" => {
                let uri = args.get("uri").and_then(|v| v.as_str()).ok_or("Missing uri")?;
                let offset = args.get("offset").and_then(|v| v.as_u64()).ok_or("Missing offset")? as usize;
                let _items = self.service.completion(uri, offset).await;
                Ok(json!([]))
            }
            "search" => {
                let query = args.get("query").and_then(|v| v.as_str()).ok_or("Missing query")?;
                let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(5) as usize;

                if let Some(searcher) = &self.searcher {
                    let _results = searcher.search(query, limit).await.map_err(|e| e.to_string())?;
                    // results is Vec<SearchResult>, need conversion
                    Ok(json!([]))
                }
                else {
                    Err("Search not supported".to_string())
                }
            }
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }

    async fn handle_notification(&self, _notification: JsonRpcNotification) {
        // Handle notifications if needed
    }
}
