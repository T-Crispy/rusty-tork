use crate::world::World;
use crate::world::room::Room;

use std::fs;
use std::ptr;

pub fn build_world(filename: String) -> World {
    let min_source_lines = 6;
    /*
    AT LEAST one line for:
    >world name
    ># of rooms
    >room id
    >room name
    >room desc
    >room pathways
     */

    let mut to_build = World{name: String::from("new world"), rooms: Vec::new()};
    
    //let mut finished = false;
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split('\n').collect();

    if lines.len() > min_source_lines {
        //get name of world
        to_build.name = lines[0].to_string();
        //get # of rooms
        let num_rooms: usize = lines[1].trim().parse::<usize>().unwrap();
        
        let mut curr_line: usize = 2;

        //get id, name, & desc of each room
        for room_iter in 0..num_rooms {
            let i = room_iter as usize;
            let mut curr_room: Room = Room {
                name: String::from(""),
                desc: String::from(""),
                id: 0,
                directions: [ptr::null_mut(), 
                            ptr::null_mut(), 
                            ptr::null_mut(),
                            ptr::null_mut()]
            };

            curr_room.id = lines[curr_line + (i * 3)].trim().parse::<usize>().unwrap();
            curr_room.name = lines[curr_line + (i * 3) + 1].to_string();
            curr_room.desc = lines[curr_line + (i * 3) + 2].to_string();
            to_build.rooms.push(curr_room);
        }

        curr_line = curr_line + (num_rooms * 3);

        //get pathways for each room
        for room_iter in 0..num_rooms {
            let i = room_iter as usize;
            
            let paths: Vec<&str> = lines[curr_line + i].trim().split('|').collect();
            for j in 0..4 {
                if paths[j] != "NULL" {
                    let num_id: usize = paths[j].trim().parse::<usize>().unwrap();
                    to_build.rooms[i].directions[j] = &mut to_build.rooms[num_id];
                }
            }
        }
    }

    to_build
}