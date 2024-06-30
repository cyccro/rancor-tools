use axum::{extract::DefaultBodyLimit, routing::post, Router};
use tower_http::cors::CorsLayer;

pub mod merger;
pub fn routes() -> Router {
    Router::new()
        .route(
            "/merge/:id",
            post(merger::process_res).layer(DefaultBodyLimit::max(1 << 32)),
        )
        .layer(CorsLayer::permissive())
}
