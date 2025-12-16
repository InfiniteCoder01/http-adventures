use crate::Item;

impl crate::Server {
    pub fn interact(&mut self, id: u32, player_id: u32) {
        let [Some(obj), Some(plr)] = self.objects.get_disjoint_mut([&id, &player_id]) else {
            return;
        };
        if plr.x != obj.x || plr.y != obj.y {
            return;
        }

        if obj.texture == "objects/pine.png" {
            self.despawn(id);
            self.give(player_id, Item::Wood, 5);
        }
    }
}
