use crate::world::room::Room;

pub struct NPC {
    pub name: String,
    pub id: usize,
    pub loc: *const Room,

    pub health: u16,
    pub hit_pwr: u16, //how much damage the NPC does to player on a hit
    pub hit_chance: f64,
}
