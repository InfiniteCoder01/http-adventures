use crate::Server;
use axum::extract::ws::*;

pub mod socket;
pub use socket::handle_socket;

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
    let persist =
        server
            .player_persist
            .entry(player_id)
            .or_insert_with(|| crate::server::PlayerData {
                inventory: std::collections::HashMap::new(),
            });
    for (item, count) in &persist.inventory {
        if *count == 0 {
            continue;
        }
        msg.push(*item as u8 + 1);
        msg.extend_from_slice(&count.to_be_bytes());
    }
    msg.push(0);

    Message::binary(msg)
}
