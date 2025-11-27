// pub mod cache;
pub mod message;
// pub mod world;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    use axum::routing::*;
    let app = axum::Router::new()
        .route(
            "/api",
            get(|ws: axum::extract::ws::WebSocketUpgrade| async {
                ws.on_upgrade(message::handle_socket)
            }),
        )
        .fallback_service(tower_http::services::ServeDir::new("frontend"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
