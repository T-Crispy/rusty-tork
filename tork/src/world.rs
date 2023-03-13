pub mod room;
pub mod character;

use room::Room;

pub struct World{
    pub name: String,
    pub rooms: Vec<Room>,
}

impl World{
    pub fn get_room_index(&self, id: usize) -> (usize, bool) {
        //make sure  the id passed is in vector bounds
        if self.rooms.len() > id {
            //see if the id matches up
            if self.rooms[id].id != id {
                //loop through all the rooms until it is found
                for i in 0..self.rooms.len(){
                    if self.rooms[i].id == id {
                        return (i as usize, true);
                    }
                }
            }
            else {
                return (id, true);
            }
        }

        (0, false)
    }
}