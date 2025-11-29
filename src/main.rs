pub mod message;

pub mod server;
pub use server::Server;

pub static SERVER: std::sync::RwLock<Option<Server>> = std::sync::RwLock::new(None);

pub fn log_err<T, E: std::fmt::Display>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(err) => {
            log::error!("{}", err);
            None
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut loader = tiled::Loader::new();
    let map = loader.load_tmx_map("maps/demo.tmx").unwrap();
    let server = Server::new(&map);
    SERVER.write().unwrap().replace(server);

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
