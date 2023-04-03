use std::ptr;

pub enum Directions{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub struct Doorway{
    pub name: String,
    pub article_phrase: String,
    pub lock: isize,
    pub path: *const Room,
}

impl Clone for Doorway{
    fn clone(&self) -> Doorway {
        let door_copy: Doorway = Doorway { 
            name: self.name.clone(), 
            article_phrase: self.article_phrase.clone(), 
            lock: self.lock,
            path: ptr::null_mut()
        };
        door_copy
    }
}

pub struct Room{
    pub name: String,
    pub desc: String,
    pub id: usize,
    pub pathways: [Doorway; 4],
}

impl Clone for Room{
    fn clone(&self) -> Room {
        let room_copy: Room = Room{
                name: self.name.clone(), 
                desc: self.desc.clone(), 
                id: self.id,
                pathways: [self.pathways[0].clone(), 
                        self.pathways[1].clone(), 
                        self.pathways[2].clone(), 
                        self.pathways[3].clone()]
            };
        room_copy
    }
}