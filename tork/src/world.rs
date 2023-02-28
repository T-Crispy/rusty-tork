pub mod room;

use room::Room;

pub struct World{
    pub name: String,
    pub rooms: Vec<Room>,
}