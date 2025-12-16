pub mod message;

pub mod server;
pub use server::Server;

pub mod item;
pub use item::Item;

pub mod behaviour;

pub static SERVER: std::sync::RwLock<Option<Server>> = std::sync::RwLock::new(None);

macro_rules! log_err {
    ($result:expr) => {
        match $result {
            Ok(value) => Some(value),
            Err(err) => {
                log::error!("{}", err);
                None
            }
        }
    };
}
use log_err;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut loader = tiled::Loader::new();
    let map = loader.load_tmx_map("maps/demo.tmx").unwrap();
    let server = Server::new(&map);
    SERVER.write().unwrap().replace(server);

    use axum::routing::*;
    use tower_http::services::ServeDir;
    let app = axum::Router::new()
        .route(
            "/api",
            get(|ws: axum::extract::ws::WebSocketUpgrade| async {
                ws.on_upgrade(message::handle_socket)
            }),
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback_service(ServeDir::new("frontend"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
