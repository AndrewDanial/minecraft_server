use crate::varint::VarInt;
#[derive(Debug, Clone, Copy)]
pub struct LoginPlay {
    pub entity_id: i32,
    pub is_hardcore: u8,
    pub dimension_count: VarInt,
    pub dimension_names: &'static str,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: VarInt,
    pub dimension_name: &'static str,
    pub hashed_seed: u64,
    pub game_mode: u8,
    pub previous_game_mode: u8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    pub portal_cooldown: VarInt,
    pub sea_level: VarInt,
    pub enforce_secure_chat: bool,
}

impl LoginPlay {
    const PROTOCOL_ID: u8 = 0x2C;
    pub fn new() -> Self {
        Self {
            entity_id: 0,
            is_hardcore: 0,
            dimension_count: VarInt(1),
            dimension_names: "minecraft:overworld",
            max_players: VarInt(2),
            view_distance: VarInt(12),
            simulation_distance: VarInt(12),
            reduced_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt(0),
            dimension_name: "minecraft:overworld",
            hashed_seed: 0x9f86d081,
            game_mode: 1,
            previous_game_mode: 1,
            is_debug: false,
            is_flat: true,
            has_death_location: false,
            portal_cooldown: VarInt(10),
            sea_level: VarInt(0),
            enforce_secure_chat: false,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(Self::PROTOCOL_ID);
        vec.extend_from_slice(&self.entity_id.to_be_bytes());
        vec.push(self.is_hardcore);
        vec.extend_from_slice(&self.dimension_count.write_varint());
        vec.push(self.dimension_names.len() as u8);
        vec.extend_from_slice(self.dimension_names.as_bytes());
        vec.extend_from_slice(&self.max_players.write_varint());
        vec.extend_from_slice(&self.view_distance.write_varint());
        vec.extend_from_slice(&self.simulation_distance.write_varint());
        vec.push(self.reduced_debug_info as u8);
        vec.push(self.enable_respawn_screen as u8);
        vec.push(self.do_limited_crafting as u8);
        vec.extend_from_slice(&self.dimension_type.write_varint());
        vec.push(self.dimension_name.len() as u8);
        vec.extend_from_slice(self.dimension_name.as_bytes());
        vec.extend_from_slice(&self.hashed_seed.to_be_bytes());
        vec.push(self.game_mode);
        vec.push(self.previous_game_mode);
        vec.push(self.is_debug as u8);
        vec.push(self.is_flat as u8);
        vec.push(self.has_death_location as u8);
        vec.extend_from_slice(&self.portal_cooldown.write_varint());
        vec.extend_from_slice(&self.sea_level.write_varint());
        vec.push(self.enforce_secure_chat as u8);
        let mut len = VarInt(vec.len() as u64).write_varint();
        len.append(&mut vec);

        len
    }
}
