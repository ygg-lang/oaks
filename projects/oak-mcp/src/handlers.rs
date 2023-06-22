#[cfg(feature = "axum")]
pub mod axum_handlers {
    use crate::{JsonRpcRequest, JsonRpcResponse, McpServer};
    use axum::{Json, Router, extract::State, routing::post};
    use oak_lsp::service::LanguageService;
    use std::sync::Arc;

    pub fn create_router<S: LanguageService + 'static>(service: S) -> Router {
        let server = Arc::new(McpServer::new(service));
        Router::new().route("/", post(handle_mcp::<S>)).with_state(server)
    }

    async fn handle_mcp<S: LanguageService + 'static>(State(server): State<Arc<McpServer<S>>>, Json(request): Json<JsonRpcRequest>) -> Json<JsonRpcResponse> {
        Json(server.handle_request(request).await)
    }
}

#[cfg(feature = "actix-web")]
pub mod actix_handlers {
    use super::*;
    use crate::{JsonRpcRequest, McpServer};
    use actix_web::{HttpResponse, Responder, web};
    use oak_lsp::service::LanguageService;
    use std::sync::Arc;

    pub fn config<S: LanguageService + 'static>(cfg: &mut web::ServiceConfig, service: S) {
        let server = Arc::new(McpServer::new(service));
        cfg.service(web::scope("/mcp").app_data(web::Data::new(server)).route("", web::post().to(handle_mcp::<S>)));
    }

    async fn handle_mcp<S: LanguageService + 'static>(server: web::Data<Arc<McpServer<S>>>, request: web::Json<JsonRpcRequest>) -> impl Responder {
        let response = server.handle_request(request.into_inner()).await;
        HttpResponse::Ok().json(response)
    }
}
