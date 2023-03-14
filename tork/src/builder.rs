use crate::world::World;
use crate::world::room::Room;

//use core::num;
use std::fs;
use std::ptr;

pub fn build_world(filename: String) -> (World, String) {
    let mut to_build = World{name: String::from("new world"), rooms: Vec::new()};
    
    //let mut finished = false;
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut lines: Vec<&str> = contents.split('\n').collect();

    let mut world_ind: isize = -1;
    let mut rooms_ind: isize = -1;
    let mut paths_ind: isize = -1;

    //check for headers
    for i in 0..lines.len() {
        //remove any existing carriage return
        lines[i] = lines[i].trim_matches('\r');
        let curr_line = lines[i];

        //check for headers
        if curr_line == "#world" {
            world_ind = i as isize;
        }
        else if curr_line == "#rooms" {
            rooms_ind = i as isize;
        }
        else if curr_line == "#paths" {
            paths_ind = i as isize;
        }
    } //end header check

    if world_ind == -1 {
        return (to_build, String::from("err: #world header missing"));
    }
    if rooms_ind == -1 {
        return (to_build, String::from("err: #rooms header missing"));
    }
    if paths_ind == -1 {
        return (to_build, String::from("err: #paths header missing"));
    }
    //return (to_build, String::from("err: # header missing"));

    //set world name
    if (rooms_ind - world_ind) == 1 {
        return (to_build, String::from("err: World name is missing"));
    }
    else {
        let world_name_ind = (world_ind + 1) as usize;
        to_build.name = lines[world_name_ind].to_string();
    }

    let rooms_ind: usize = rooms_ind as usize;
    let paths_ind: usize = paths_ind as usize;
    let mut rooms_vec = vec![""; paths_ind - rooms_ind - 1];
    let mut paths_vec = vec![""; lines.len() - paths_ind - 1];

    //make sure the # of lines between the headers = 3r + 1
    //wher r is the # of rooms
    if (paths_ind - rooms_ind - 2) % 3 == 0 {
        //get the sub_arr for the room and path stuff
        let left_ind = rooms_ind+1;
        let right_ind = paths_ind;
        rooms_vec.clone_from_slice(&lines[left_ind..right_ind]);

        //get number of rooms
        let num_rooms: usize = rooms_vec[0].trim().parse::<usize>().unwrap();
        //loop through the room lines to get the id,name, & desc of each room
        for i in 0..num_rooms{
            let offset: usize = (i * 3) + 1;
            let mut curr_room: Room = Room {
                name: String::from(""),
                desc: String::from(""),
                id: 0,
                pathways: [ptr::null_mut(), 
                            ptr::null_mut(), 
                            ptr::null_mut(),
                            ptr::null_mut()]
            };
            curr_room.id = rooms_vec[offset].trim().parse::<usize>().unwrap();
            curr_room.name = rooms_vec[offset + 1].to_string();
            curr_room.desc = rooms_vec[offset + 2].to_string();
            to_build.rooms.push(curr_room);
        }

        paths_vec.clone_from_slice(&lines[paths_ind+1..]);
        if paths_vec.len() < num_rooms {
            return (to_build, String::from("err: That are less pathway sets in file than there are number of rooms."));
        }

        //loop through path lines to set pathways
        for i in 0..num_rooms {
            //get all the paths given
            let paths: Vec<&str> = paths_vec[i].trim().split('|').collect();

            //make sure 4 pathways were given
            if paths.len() < 4{
                let mut message = String::from("err: there are less than 4 paths in path set #");
                message.push_str(i.to_string().as_str());
                return (to_build, message);
            }

            //set pathways for 
            for j in 0..4{
                if paths[j] != "NULL" {
                    let num_id: usize = paths[j].trim().parse::<usize>().unwrap();
                    let room_id: usize = to_build.get_room_index(num_id).0;
                    to_build.rooms[i].pathways[j] = &mut to_build.rooms[room_id];
                }
            }
        }
    }
    else {
        return (to_build, String::from("err: number of lines between #rooms and #paths headers are off. Double check the room lines"));
    }

    /*
    //~~~~~~~~~~~~~~~~~~prev code~~~~~~~~~~~~~~~~~~~~~
    if lines.len() > min_source_lines {
        //get name of world
        to_build.name = lines[0].trim_matches('\r').to_string();
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
                pathways: [ptr::null_mut(), 
                            ptr::null_mut(), 
                            ptr::null_mut(),
                            ptr::null_mut()]
            };

            curr_room.id = lines[curr_line + (i * 3)].trim().parse::<usize>().unwrap();
            curr_room.name = lines[curr_line + (i * 3) + 1].trim_matches('\r').to_string();
            curr_room.desc = lines[curr_line + (i * 3) + 2].trim_matches('\r').to_string();
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
                    let room_id: usize = to_build.get_room_index(num_id).0;
                    to_build.rooms[i].pathways[j] = &mut to_build.rooms[room_id];
                }
            }
        }
    }*/

    (to_build,String::from("success"))
}