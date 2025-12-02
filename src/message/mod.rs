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

    crate::log_err!(tx.send(texture("spritesheet.png")).await);

    let (msg, player_id) = {
        let mut server = crate::SERVER.write().unwrap();
        let server = server.as_mut().unwrap();
        let player_id = server.spawn(crate::server::Object {
            x: 0.0,
            y: 0.0,
            client: None,
        });
        (join(server), player_id)
    };
    crate::log_err!(tx.send(msg).await);

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
                    let buf = {
                        let mut server = crate::SERVER.write().unwrap();
                        let server = server.as_mut().unwrap();

                        let player = server.objects.get_mut(&player_id).unwrap();
                        let last_pos = (player.x, player.y);

                        player.x = f32::from_be_bytes(bytes[1..5].try_into().unwrap());
                        player.y = f32::from_be_bytes(bytes[5..9].try_into().unwrap());

                        let mut buf = vec![b'u'];
                        let pos = (player.x, player.y);
                        server.update(&mut buf, pos, Some(last_pos));
                        buf
                    };
                    if buf.len() > 2 {
                        crate::log_err!(tx.send(Message::binary(buf)).await);
                    }
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
