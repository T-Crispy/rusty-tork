use std::ptr;

pub enum Directions{
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub struct Doorway{
    pub name: String, //simple noun for the "door"
    pub pres_phrase: String, //presence phrase ex.) "there is a" or "there are"
    pub lock: usize,
    pub path: *const Room,
}

impl Clone for Doorway{
    fn clone(&self) -> Doorway {
        let door_copy: Doorway = Doorway { 
            name: self.name.clone(), 
            pres_phrase: self.pres_phrase.clone(), 
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
    pub dark: bool,
    pub floor: [isize; 7], //ids of items on floor (-1 for empty slot)
}

impl Room{
    pub fn catch_item(&mut self, item_id: usize) -> bool {
        for (i, item) in self.floor.iter().enumerate() {
            if item.eq(&-1) {
                let id = item_id.clone() as isize;
                let _ = &self.floor[i].clone_from(&id);
                return true;
            }
        }
        //no room on floor
        false
    }

    pub fn handoff_item(&mut self, item_id: usize) -> bool {
        for (i, item) in self.floor.iter().enumerate() {
            //compare Item IDs
            if item.eq(&(item_id as isize)) {
                self.floor[i] = -1;
                return true;
            }
        }
        return false
    }

    pub fn floor_full(&self) -> bool {
        for item in self.floor.iter() {
            if item.eq(&-1) {
                return false;
            }
        }
        return true
    }
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
                        self.pathways[3].clone()],
                dark: self.dark,
                floor: self.floor.clone(),
        };
        room_copy
    }
}