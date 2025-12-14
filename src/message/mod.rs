use crate::Server;
use axum::extract::ws::*;

pub type Client = tokio::sync::mpsc::UnboundedSender<Message>;

pub fn join(server: &mut Server, player_id: u32) -> Message {
    let mut msg = Vec::new();
    msg.push(b'j');
    msg.extend_from_slice(&Server::CHUNK_SIZE.to_be_bytes());
    msg.extend_from_slice(server.tileset.as_bytes());
    msg.push(0);
    msg.extend_from_slice(&server.tile_size.to_be_bytes());
    msg.extend_from_slice(&(server.offsets.len() as u32).to_be_bytes());
    for offset in &server.offsets {
        msg.extend_from_slice(&offset.to_be_bytes());
    }
    let player = &server.objects[&player_id];
    server.update(&mut msg, (player.x, player.y), None);
    msg.extend_from_slice(&player_id.to_be_bytes());
    Message::binary(msg)
}

pub async fn handle_socket(socket: WebSocket) {
    use futures_util::{SinkExt, StreamExt};

    let (mut tx1, mut rx) = socket.split();
    let (tx, mut rx1) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(message) = rx1.recv().await {
            crate::log_err!(tx1.send(message).await);
        }
        crate::log_err!(tx1.close().await);
    });

    let player_id = {
        let mut server = crate::SERVER.write().unwrap();
        let server = server.as_mut().unwrap();
        let player_id = server.spawn(crate::server::Object {
            x: 16,
            y: 16,
            texture: "Kaleb.png".to_owned(),
            client: Some(tx.clone()),
        });

        crate::log_err!(tx.send(join(server, player_id)));
        player_id
    };

    while let Some(msg) = rx.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(err) => {
                log::error!("{err}");
                break;
            }
        };

        match msg {
            Message::Text(msg) => log::info!("Received message: {}", msg),
            Message::Binary(bytes) => {
                if bytes[0] == b'u' {
                    let mut server = crate::SERVER.write().unwrap();
                    let server = server.as_mut().unwrap();

                    let new_pos = (
                        u32::from_be_bytes(bytes[1..5].try_into().unwrap()),
                        u32::from_be_bytes(bytes[5..9].try_into().unwrap()),
                    );

                    let mut buf = vec![b'u'];
                    server.update(&mut buf, new_pos, Some(player_id));
                    server.move_object(player_id, new_pos);

                    // Check against an empty packet size
                    if buf.len() > 3 {
                        crate::log_err!(tx.send(Message::binary(buf)));
                    }
                } else if bytes[0] == b'i' {
                    let mut server = crate::SERVER.write().unwrap();
                    let server = server.as_mut().unwrap();

                    let id = u32::from_be_bytes(bytes[1..5].try_into().unwrap());
                    server.interact(id, player_id);
                }
            }
            Message::Close(_) => {
                println!("Closing WebSocket connection.");
                break;
            }
            _ => {}
        }
    }

    let mut server = crate::SERVER.write().unwrap();
    let server = server.as_mut().unwrap();
    server.despawn(player_id);
}
