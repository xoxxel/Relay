pub mod routes_files;

use crate::state::AppState;
use crate::web_assets::static_handler;
use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tower_http::cors::{Any, CorsLayer};

/// Architecture placeholder: optional PIN check middleware (no-op for now as per section 11)
async fn pin_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // In future phases: check X-Pin header if PIN protection is active in settings
    Ok(next.run(req).await)
}

pub fn create_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    let api_routes = Router::new()
        .route("/files", get(routes_files::list_files))
        .layer(middleware::from_fn(pin_middleware));

    Router::new()
        .nest("/api", api_routes)
        .fallback(static_handler)
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

