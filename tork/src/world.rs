pub mod room;
pub mod character;
pub mod item;

use room::Room;
use item::Item;

pub struct World{
    pub name: String,
    pub rooms: Vec<Room>,
    pub items: Vec<Item>,
    pub grue_enabled: bool,
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

    pub fn get_item_loc(&self, id: usize) -> (isize, bool){
        let result = self.get_item_index(id);
        if result.1 == true {
            return (self.items[result.0].loc, true);
        }

        (-2, false)
    }

    pub fn fetch_item_id(&self, name: &String) -> (usize, bool){
        for item in self.items.iter(){
            if item.name.eq(name){
                return (item.id, true);
            }
        }

        (0, false)
    }
}