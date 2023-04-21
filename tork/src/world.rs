pub mod room;
pub mod player;
pub mod item;
pub mod npc;

use room::Room;
use item::Item;
use player::Player;
use npc::NPC;

use self::item::ItemType;

pub enum State{
    ERROR,
    QUIT,
    DEATH,
    SAVE,
    RESTART,
}

pub struct World{
    pub name: String,
    pub rooms: Vec<Room>,
    pub items: Vec<Item>,
    pub npcs: Vec<NPC>,
    pub grue_enabled: bool,
    pub player: Player,
}

impl World{
    pub fn get_room_index(&self, id: usize) -> (usize, bool) {

        //quick check in case the ID matches the index
        if id < self.rooms.len() {
            if self.rooms[id].id == id{
                return (id, true);
            }
        }

        //need to scan the list of rooms
        for (i, room) in self.rooms.iter().enumerate() {
            if room.id == id {
                return (i, true);
            }
        }

        //no results
        (0, false)
    }

    pub fn get_item_index(&self, id: usize) -> (usize, bool) {
        //quick check in case the ID matches the index
        if (id) < self.items.len() {
            if self.items[id].id == id{
                return (id, true);
            }
        }

        //need to scan the list of rooms
        for (i, item) in self.items.iter().enumerate() {
            if item.id == id {
                return (i, true);
            }
        }

        (0, false)
    }

    pub fn get_npc_index(&self, id: usize) -> (usize, bool) {
        //quick check in case the ID matches the index
        if (id) < self.npcs.len() {
            if self.npcs[id].id == id{
                return (id, true);
            }
        }

        //need to scan the list of rooms
        for (i, npc) in self.npcs.iter().enumerate() {
            if npc.id == id {
                return (i, true);
            }
        }

        (0, false)
    }

    pub fn find_first_item_type(&self, type_to_find: ItemType) -> (usize, bool) {
        for item in self.player.inv.iter() {
            let item_ind = self.get_item_index(*item as usize).0;
            if self.items[item_ind].item_type.eq(&type_to_find) {
                return (item_ind, true)
            }
        }

        (0, false)
    }

    pub fn get_item_loc(&self, id: usize) -> (isize, bool){
        let result = self.get_item_index(id);
        if result.1 == true {
            return (self.items[result.0].loc, true);
        }

        (-2, false)
    }

    pub fn fetch_item_id(&self, name: &String) -> (usize, bool){
        for item in self.items.iter(){
            if item.name.to_ascii_lowercase().eq(name){
                return (item.id, true);
            }
        }

        (0, false)
    }

    pub fn fetch_npc_id(&self, name: &String) -> (usize, bool) {
        for npc in self.npcs.iter() {
            if npc.name.to_ascii_lowercase().eq(name){
                return (npc.id, true)
            }
        }

        (0, false)
    }
}