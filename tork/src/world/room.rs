pub enum Directions{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub struct Room{
    pub name: String,
    pub desc: String,
    pub id: u32,

    //directions
    pub north: u32,
    pub east: u32,
    pub south: u32,
    pub west: u32,
    pub directions: [*mut Room; 4],
}