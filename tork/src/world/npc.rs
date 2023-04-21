use crate::world::room::Room;

pub struct NPC {
    pub id: usize,
    pub loc: *const Room,
    pub name: String,
    pub health: u16,
    pub hit_pwr: u16, //how much damage the NPC does to player on a hit
    pub hit_chance: u16,
}

impl Clone for NPC{
    fn clone(&self) -> NPC {
        let npc_clone: NPC = NPC { 
            id: self.id, 
            loc: self.loc, 
            name: self.name.clone(), 
            health: self.health, 
            hit_pwr: self.hit_pwr, 
            hit_chance: self.hit_chance, 
        };

        npc_clone
    }
}
