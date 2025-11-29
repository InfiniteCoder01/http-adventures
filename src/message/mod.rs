use crate::Server;
use axum::extract::ws::*;

pub type Client = futures_util::stream::SplitSink<WebSocket, Message>;

pub fn texture(texture: &str) -> Message {
    // TODO: caching
    let mut msg = Vec::new();
    msg.push(b't');
    msg.extend_from_slice(texture.as_bytes());
    msg.push(0);
    msg.extend(std::fs::read(std::path::Path::new("textures/").join(texture)).unwrap());
    Message::binary(msg)
}

pub fn join(server: &Server) -> Message {
    let mut msg = Vec::new();
    msg.push(b'j');
    msg.extend_from_slice(&Server::CHUNK_SIZE.to_be_bytes());
    msg.extend_from_slice(&server.tile_size.to_be_bytes());
    server.update(&mut msg, (0.0, 0.0), None);
    Message::binary(msg)
}

pub async fn handle_socket(socket: WebSocket) {
    use futures_util::{SinkExt, StreamExt};
    let (mut tx, mut rx) = socket.split();

    let msg = texture("spritesheet.png");
    crate::log_err(tx.send(msg).await);

    let msg = join(crate::SERVER.read().unwrap().as_ref().unwrap());
    crate::log_err(tx.send(msg).await);

    // let player = crate::SERVER.lock().unwrap().join(tx);
    // crate::log_err(
    //     player
    //         .write()
    //         .unwrap()
    //         .client
    //         .as_mut()
    //         .unwrap()
    //         .start_send_unpin(texture("test.png")),
    // );

    while let Some(msg) = rx.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(err) => {
                log::error!("{err}");
                // crate::log_err(
                //     player
                //         .write()
                //         .unwrap()
                //         .client
                //         .as_mut()
                //         .unwrap()
                //         .close()
                //         .await,
                // );
                break;
            }
        };

        match msg {
            Message::Text(msg) => log::info!("Received message: {}", msg),
            Message::Binary(bytes) => {
                if bytes[0] == b'u' {
                    let x = f32::from_be_bytes(bytes[1..5].try_into().unwrap());
                    let y = f32::from_be_bytes(bytes[5..9].try_into().unwrap());
                    println!("{x}, {y}");
                }
            }
            Message::Close(_) => {
                println!("Closing WebSocket connection.");
                break;
            }
            _ => {}
        }
    }
}
