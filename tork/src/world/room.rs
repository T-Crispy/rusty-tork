use std::ptr;

pub enum Directions{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

//#[derive(Copy, Clone)]
pub struct Room{
    pub name: String,
    pub desc: String,
    pub id: usize,

    //directions
    //pub north: u32,
    //pub east: u32,
    //pub south: u32,
    //pub west: u32,
    pub pathways: [*const Room; 4],

}

impl Clone for Room{
    fn clone(&self) -> Room {
        let room_copy: Room = Room{name: self.name.clone(), desc: self.desc.clone(), id: self.id, pathways: [ptr::null_mut(),ptr::null_mut(),ptr::null_mut(),ptr::null_mut()]};
        room_copy
    }
}

/*
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
*/