use core::range::Range;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UriRequest {
    pub uri: String,
}

#[derive(Deserialize)]
pub struct RangeRequest {
    pub uri: String,
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
}

#[derive(Deserialize)]
pub struct PositionRequest {
    pub uri: String,
    pub position: usize,
}

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

#[derive(Deserialize)]
pub struct RenameRequest {
    pub uri: String,
    #[serde(with = "oak_core::serde_range")]
    pub range: Range<usize>,
    pub new_name: String,
}

#[cfg(feature = "axum")]
pub mod axum_handlers {
    use crate::{service::LanguageService, types::*};
    use axum::{Json, Router, extract::State, routing::post};
    use std::sync::Arc;

    pub fn create_router<S: LanguageService + 'static>(service: S) -> Router {
        Router::new()
            .route("/hover", post(hover::<S>))
            .route("/folding", post(folding::<S>))
            .route("/symbols", post(symbols::<S>))
            .route("/workspace_symbols", post(workspace_symbols::<S>))
            .route("/definition", post(definition::<S>))
            .route("/references", post(references::<S>))
            .route("/rename", post(rename::<S>))
            .route("/completion", post(completion::<S>))
            .route("/diagnostics", post(diagnostics::<S>))
            .with_state(Arc::new(service))
    }

    async fn hover<S: LanguageService>(State(service): State<Arc<S>>, Json(payload): Json<crate::RangeRequest>) -> Json<Option<Hover>> {
        Json(service.hover(&payload.uri, payload.range).await)
    }

    async fn folding<S: LanguageService>(State(service): State<Arc<S>>, Json(payload): Json<crate::UriRequest>) -> Json<Vec<FoldingRange>> {
        Json(service.folding_ranges(&payload.uri).await)
    }

    async fn symbols<S: LanguageService>(State(service): State<Arc<S>>, Json(payload): Json<crate::UriRequest>) -> Json<Vec<StructureItem>> {
        Json(service.document_symbols(&payload.uri).await)
    }

    async fn workspace_symbols<S: LanguageService>(State(service): State<Arc<S>>, Json(payload): Json<crate::QueryRequest>) -> Json<Vec<WorkspaceSymbol>> {
        Json(service.workspace_symbols(&payload.query).await)
    }

    async fn definition<S: LanguageService>(State(service): State<Arc<S>>, Json(payload): Json<crate::RangeRequest>) -> Json<Vec<LocationRange>> {
        let locs = service.definition(&payload.uri, payload.range).await;
        Json(locs.into_iter().map(LocationRange::from).collect())
    }

    async fn references<S: LanguageService>(State(service): State<Arc<S>>, Json(payload): Json<crate::RangeRequest>) -> Json<Vec<LocationRange>> {
        let locs = service.references(&payload.uri, payload.range).await;
        Json(locs.into_iter().map(LocationRange::from).collect())
    }

    async fn rename<S: LanguageService>(State(service): State<Arc<S>>, Json(payload): Json<crate::RenameRequest>) -> Json<Option<WorkspaceEdit>> {
        Json(service.rename(&payload.uri, payload.range, payload.new_name).await)
    }

    async fn completion<S: LanguageService>(State(service): State<Arc<S>>, Json(payload): Json<crate::PositionRequest>) -> Json<Vec<CompletionItem>> {
        Json(service.completion(&payload.uri, payload.position).await)
    }

    async fn diagnostics<S: LanguageService>(State(service): State<Arc<S>>, Json(payload): Json<crate::UriRequest>) -> Json<Vec<Diagnostic>> {
        Json(service.diagnostics(&payload.uri).await)
    }
}

#[cfg(feature = "actix-web")]
pub mod actix_handlers {
    use super::*;
    use crate::{service::LanguageService, types::*};
    use actix_web::{HttpResponse, Responder, web};
    use std::sync::Arc;

    pub fn config<S: LanguageService + 'static>(cfg: &mut web::ServiceConfig, service: S) {
        let service = Arc::new(service);
        cfg.service(
            web::scope("/lsp")
                .app_data(web::Data::from(service))
                .route("/hover", web::post().to(hover::<S>))
                .route("/folding", web::post().to(folding::<S>))
                .route("/symbols", web::post().to(symbols::<S>))
                .route("/workspace_symbols", web::post().to(workspace_symbols::<S>))
                .route("/definition", web::post().to(definition::<S>))
                .route("/references", web::post().to(references::<S>))
                .route("/rename", web::post().to(rename::<S>)),
        );
    }

    async fn hover<S: LanguageService>(service: web::Data<Arc<S>>, payload: web::Json<RangeRequest>) -> impl Responder {
        HttpResponse::Ok().json(service.hover(&payload.uri, payload.range.clone()).await)
    }

    async fn folding<S: LanguageService>(service: web::Data<Arc<S>>, payload: web::Json<UriRequest>) -> impl Responder {
        HttpResponse::Ok().json(service.folding_ranges(&payload.uri).await)
    }

    async fn symbols<S: LanguageService>(service: web::Data<Arc<S>>, payload: web::Json<UriRequest>) -> impl Responder {
        HttpResponse::Ok().json(service.document_symbols(&payload.uri).await)
    }

    async fn workspace_symbols<S: LanguageService>(service: web::Data<Arc<S>>, payload: web::Json<QueryRequest>) -> impl Responder {
        HttpResponse::Ok().json(service.workspace_symbols(&payload.query).await)
    }

    async fn definition<S: LanguageService>(service: web::Data<Arc<S>>, payload: web::Json<RangeRequest>) -> impl Responder {
        let locs = service.definition(&payload.uri, payload.range).await;
        let locs: Vec<crate::types::LocationRange> = locs.into_iter().map(Into::into).collect();
        HttpResponse::Ok().json(locs)
    }

    async fn references<S: LanguageService>(service: web::Data<Arc<S>>, payload: web::Json<RangeRequest>) -> impl Responder {
        let locs = service.references(&payload.uri, payload.range).await;
        let locs: Vec<crate::types::LocationRange> = locs.into_iter().map(Into::into).collect();
        HttpResponse::Ok().json(locs)
    }

    async fn rename<S: LanguageService>(service: web::Data<Arc<S>>, payload: web::Json<RenameRequest>) -> impl Responder {
        HttpResponse::Ok().json(service.rename(&payload.uri, payload.range.clone(), payload.new_name.clone()).await)
    }
}
