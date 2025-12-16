use std::collections::HashMap;

pub struct PlayerData {
    pub inventory: HashMap<crate::Item, u32>,
}

impl super::Server {
    pub fn give(&mut self, player_id: u32, item: crate::Item, count: u32) {
        let Some(persist) = self.player_persist.get_mut(&player_id) else {
            log::error!("tried to give {count} {item} to unknown player #{player_id}");
            return;
        };

        let stack = persist.inventory.entry(item).or_insert(0);
        *stack += count;

        self.objects
            .get(&player_id)
            .and_then(|player| player.client.as_ref())
            .map(|client| {
                let mut message = vec![b'i'];
                message.push(item as u8 + 1);
                message.extend_from_slice(&stack.to_be_bytes());
                crate::log_err!(client.send(axum::extract::ws::Message::binary(message)));
            });
    }
}
