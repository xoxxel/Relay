pub mod routes_clip;
pub mod routes_files;
pub mod ws;

use crate::state::AppState;
use crate::web_assets::static_handler;
use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Request},
    http::{Method, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tower_http::cors::{Any, CorsLayer};

async fn pin_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // No-op middleware for future optional PIN verification
    Ok(next.run(req).await)
}

pub fn create_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    let file_routes = Router::new()
        .route("/files", get(routes_files::list_files).delete(routes_files::delete_file))
        .route("/files/upload", post(routes_files::upload_files))
        .route("/files/download", get(routes_files::download_file))
        .route("/files/mkdir", post(routes_files::make_directory));

    let clip_routes = Router::new()
        .route("/clips", get(routes_clip::list_clips).post(routes_clip::create_clip))
        .route("/clips/{id}", delete(routes_clip::delete_clip));

    let api_routes = Router::new()
        .merge(file_routes)
        .merge(clip_routes)
        .layer(middleware::from_fn(pin_middleware));

    Router::new()
        .nest("/api", api_routes)
        .route("/ws", get(ws::ws_handler))
        .fallback(static_handler)
        .layer(DefaultBodyLimit::max(500 * 1024 * 1024)) // 500 MB upload limit
        .layer(cors)
        .with_state(app_state)
}

pub async fn run_server(
    app_state: AppState,
    port: u16,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    let app = create_router(app_state);

    println!("Relay Axum Server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown_rx.await;
            println!("Relay Axum Server received shutdown signal");
        })
        .await?;

    Ok(())
}
