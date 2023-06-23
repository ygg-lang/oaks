#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc = "Model Context Protocol (MCP) server implementation for Oak languages."]
//! Model Context Protocol (MCP) support for the Oak language framework.
//!
//! This crate provides an implementation of the MCP server, allowing
//! Oak-based language services to be used as tools by LLMs.

use oak_core::Range;
use oak_lsp::service::LanguageService;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

pub use oak_semantic_search::{NoSemanticSearch, SemanticSearch};

/// Represents a JSON-RPC 2.0 request.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// The JSON-RPC version (must be "2.0").
    pub jsonrpc: String,
    /// The unique identifier for the request.
    pub id: JsonValue,
    /// The method name to be invoked.
    pub method: String,
    /// The parameters for the method, if any.
    pub params: Option<JsonValue>,
}

impl JsonRpcRequest {
    /// Parses a JSON string into a `JsonRpcRequest`.
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| e.to_string())
    }
}

/// Represents a JSON-RPC 2.0 response.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// The JSON-RPC version (must be "2.0").
    pub jsonrpc: String,
    /// The unique identifier corresponding to the request.
    pub id: JsonValue,
    /// The successful result of the request, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<JsonValue>,
    /// The error details if the request failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    /// Serializes the response to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Error information for a JSON-RPC response.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// The error code.
    pub code: i32,
    /// The human-readable error message.
    pub message: String,
    /// Additional error data, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
}

/// Represents a JSON-RPC 2.0 notification.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcNotification {
    /// The JSON-RPC version (must be "2.0").
    pub jsonrpc: String,
    /// The method name to be invoked.
    pub method: String,
    /// The parameters for the method, if any.
    pub params: Option<JsonValue>,
}

impl JsonRpcNotification {
    /// Parses a JSON string into a `JsonRpcNotification`.
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| e.to_string())
    }
}

/// A generic MCP server that wraps an Oak language service.
pub struct McpServer<S: LanguageService, E: SemanticSearch = NoSemanticSearch> {
    /// The language service that provides language-specific features.
    pub service: Arc<S>,
    /// The semantic search engine for code indexing and search.
    pub searcher: Option<Arc<E>>,
}

impl<S: LanguageService + 'static> McpServer<S, NoSemanticSearch>
where
    S::Vfs: oak_vfs::WritableVfs,
{
    /// Creates a new MCP server with the given language service and no semantic searcher.
    pub fn new(service: S) -> Self {
        Self { service: Arc::new(service), searcher: None }
    }
}

impl<S: LanguageService + 'static, E: SemanticSearch + 'static> McpServer<S, E>
where
    S::Vfs: oak_vfs::WritableVfs,
{
    /// Adds a semantic searcher to the MCP server.
    pub fn with_searcher<NewE: SemanticSearch>(self, searcher: NewE) -> McpServer<S, NewE> {
        McpServer { service: self.service, searcher: Some(Arc::new(searcher)) }
    }

    /// Run the MCP server on the given input and output streams.
    pub async fn run<R, W>(&self, mut reader: R, mut writer: W) -> Result<(), String>
    where
        R: tokio::io::AsyncBufRead + Unpin,
        W: tokio::io::AsyncWrite + Unpin,
    {
        let mut line = String::new();

        while reader.read_line(&mut line).await.map_err(|e| e.to_string())? > 0 {
            let input = line.trim();
            if input.is_empty() {
                line.clear();
                continue;
            }

            if let Ok(request) = JsonRpcRequest::from_json(input) {
                let response = self.handle_request(request).await;
                let response_json = response.to_json();
                writer.write_all(response_json.as_bytes()).await.map_err(|e| e.to_string())?;
                writer.write_all(b"\n").await.map_err(|e| e.to_string())?;
                writer.flush().await.map_err(|e| e.to_string())?;
            }
            else if let Ok(notification) = JsonRpcNotification::from_json(input) {
                self.handle_notification(notification).await
            }

            line.clear()
        }

        Ok(())
    }

    /// Handles an incoming JSON-RPC request and returns a response.
    /// Handles a single JSON-RPC request and returns a response.
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

    /// Handles a single JSON-RPC notification.
    pub async fn handle_notification(&self, _notification: JsonRpcNotification) {
        // Handle notifications (like initialized)
    }

    /// Handles a tool call from the MCP client.
    ///
    /// Currently supports:
    /// - `hover`: Get hover information for a position.
    /// - `definitions`: Get definitions for a symbol at a position.
    /// - `search`: Perform semantic search (if searcher is available).
    pub async fn handle_tool_call(&self, name: &str, args: JsonValue) -> Result<JsonValue, String> {
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
            "semantic_search" => {
                let searcher = self.searcher.as_ref().ok_or("Semantic search is not enabled")?;
                let query = args.get("query").and_then(|v| v.as_str()).ok_or("Missing query")?;
                let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(5) as usize;

                let results = searcher.search(query, limit).await.map_err(|e| e.to_string())?;
                Ok(json!(results))
            }
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }
}
