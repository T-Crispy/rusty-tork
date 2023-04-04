use crate::world::World;
use crate::world::room::Doorway;
use crate::world::room::Room;

//use core::num;
use std::fs;
use std::ptr;

pub fn build_world(filename: String) -> (World, String) {
    let mut build_msg = String::from("");
 
    let room_dec_lines: usize = 8;
    let num_paths: usize = 4;
    let north_offset = 3;
    let east_offset = 4;
    let south_offset = 5;
    let west_offset = 6;

    let item_dec_lines: usize = 4;

    let headers: Vec<&str> = vec!["#world","#rooms", "#items"];

    let mut to_build = World{name: String::from("new world"), rooms: Vec::new(), grue_enabled: false};
    
    //let mut finished = false;
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();

    let mut world_ind: isize = -1;

    let mut rooms_ind: isize = -1;
    let mut rooms_end: usize = 0;

    let mut items_ind: isize = -1;
    let mut items_end: usize = 0;

    //check for headers
    let mut i: usize = 0;
    //for mut i in 0..lines.len()
    while i < lines.len() {
        //remove any existing carriage return
        let mut curr_line = lines[i];

        //check for headers
        if curr_line == "#world" {
            world_ind = i as isize;

            if i < lines.len() - 1 {
                curr_line = lines[i + 1];

                if curr_line.chars().nth(0).unwrap() == '#' {
                    return (to_build, String::from("err: World name is missing"));
                }
                else{
                    to_build.name = lines[(world_ind + 1) as usize].to_string();
                }
            }

            i += 1;
        }
        else if curr_line == "#rooms" {
            rooms_ind = i as isize;

            i += 1;
            curr_line = lines[i];

            while !(headers.contains(&curr_line)) && i < lines.len(){
                //continue iterating until the next header is reached or the end of line
                i += 1;
            }

            i -= 1;
            rooms_end = i;
        }
        else if curr_line == "#items" {
            items_ind = i as isize;

            i += 1;
            curr_line = lines[i];

            while !(headers.contains(&curr_line)) && i < lines.len() {
                //continue iterating until the next header
                i += 1;
            }

            i -= 1;
            items_end = i;
        }
        i += 1;
    } //end header check

    if world_ind == -1 {
        return (to_build, String::from("err: #world header missing"));
    }
    if rooms_ind == -1 {
        return (to_build, String::from("err: #rooms header missing"));
    }
    if items_ind == -1{
        return (to_build, String::from("err: #items header missing"));
    }

    let rooms_ind: usize = rooms_ind as usize;
    let num_rooms: usize = rooms_end - rooms_ind;
    let mut rooms_vec = vec![""; num_rooms];

    let items_ind: usize = items_ind as usize;
    let num_items: usize = items_end - items_ind;
    let mut items_vec = vec![""; num_items];

    //~~~~~~~fecthing room info~~~~~~~~~
    //make sure the # of lines between the headers = 3r + 1
    //wher r is the # of rooms
    if num_rooms % room_dec_lines == 0 {
        //get the sub_arr for the room and path stuff
        let left_ind = rooms_ind+1;
        let right_ind = rooms_end;
        rooms_vec.clone_from_slice(&lines[left_ind..right_ind+1]);

        //get number of rooms
        let num_rooms: usize = rooms_vec.len() / room_dec_lines;

        //initialize to_build's room vector
        let def_door: Doorway = Doorway { name: String::from(""), 
            pres_phrase: String::from(""), 
            lock: 0, 
            path: ptr::null_mut() };
        
        let null_room: Room = Room {
            name: String::from(""),
            desc: String::from(""),
            id: 0,
            pathways: [ def_door.clone(), 
                        def_door.clone(), 
                        def_door.clone(),
                        def_door.clone()],
            dark: false
        };
        to_build.rooms = vec![null_room; num_rooms];

        let mut paths_vec = vec![vec!["";5]; num_rooms];

        //loop through the room lines to get the id,name, & desc of each room
        for i in 0..num_rooms{
            let offset: usize = i * room_dec_lines;

            //get room ID
            if rooms_vec[offset].starts_with('#') {
                let id_parse_result = rooms_vec[offset].trim().trim_start_matches('#').parse::<usize>();
                if id_parse_result.is_err() {
                    let mut err_msg = String::from("err: Could not parse the ID number for room ");
                    err_msg.push_str(offset.to_string().as_str());
                    return (to_build, err_msg);
                }
                to_build.rooms[i].id = id_parse_result.unwrap();
            }
            
            //get room name and desc
            to_build.rooms[i].name = rooms_vec[offset + 1].to_string();
            to_build.rooms[i].desc = rooms_vec[offset + 2].to_string();

            //get the lit property
            let lit_prop = rooms_vec[offset + 7].trim();
            if lit_prop == "LIT"{
                to_build.rooms[i].dark = false;
            }
            else if lit_prop == "DARK"{
                to_build.rooms[i].dark = true;
            }
            else{
                let mut err_msg = String::from("err: LIT property not correctly defined for room ");
                err_msg.push_str(offset.to_string().as_str());
                return (to_build, err_msg);
            }

            //get room pathways
            paths_vec[i][0] = rooms_vec[offset + north_offset].trim();
            paths_vec[i][1] = rooms_vec[offset + east_offset].trim();
            paths_vec[i][2] = rooms_vec[offset + south_offset].trim();
            paths_vec[i][3] = rooms_vec[offset + west_offset].trim();

        }//end for i loop

        //loop through rooms to set pathways
        for i in 0..num_rooms{

            //set pathways for 
            for j in 0..num_paths{
                //       0   ,         1      ,     2    ,      3     ,    4
                // ~direction, presence phrase, path name, lock number, path ID 
                let door_vec: Vec<&str> = paths_vec[i][j].split('|').collect();

                if door_vec.len() != 5{
                    let mut err_msg = String::from("err: defined path way #");
                    err_msg.push_str(j.to_string().as_str());
                    err_msg.push_str(" for room #");
                    err_msg.push_str(to_build.rooms[i].id.to_string().as_str());
                    err_msg.push_str(" is not formatted correctly.");
                    build_msg.push_str(err_msg.as_str());
                }

                if door_vec[4] != "NULL"{
                    let num_id: usize = door_vec[4].trim().parse::<usize>().unwrap();
                    
                    //try to set path
                    let get_room_result: (usize, bool) = to_build.get_room_index(num_id);
                    if !get_room_result.1 {
                        let mut warn_msg = String::from("warn: a path way for room #");
                        warn_msg.push_str(to_build.rooms[i].id.to_string().as_str());
                        warn_msg.push_str(" could not be linked to its defined room\n");
                        build_msg.push_str(warn_msg.as_str());
                    }
                    else {
                        let room_id: usize = get_room_result.0;
                        let dir_ind: usize;

                        //get the pathway direction
                        if door_vec[0].trim() == "~N" {
                            dir_ind = 0;
                        }
                        else if door_vec[0].trim() == "~E" {
                            dir_ind = 1;
                        }
                        else if door_vec[0].trim() == "~S" {
                            dir_ind = 2;
                        }
                        else if door_vec[0].trim() == "~W" {
                            dir_ind = 3;
                        }
                        else{
                            dir_ind = 0;

                            let mut err_msg = String::from("err: direction of defined path way #");
                            err_msg.push_str(j.to_string().as_str());
                            err_msg.push_str(" for room #");
                            err_msg.push_str(to_build.rooms[i].id.to_string().as_str());
                            err_msg.push_str(" is not valid. Must be N/E/S/W and prefixed by a \'~\'");
                            build_msg.push_str(err_msg.as_str());
                        }
                        to_build.rooms[i].pathways[dir_ind].path = &mut to_build.rooms[room_id];

                        //get other doorway stuff
                        to_build.rooms[i].pathways[dir_ind].pres_phrase = door_vec[1].trim().to_string();
                        to_build.rooms[i].pathways[dir_ind].name = door_vec[2].trim().to_string();
                        to_build.rooms[i].pathways[dir_ind].lock = door_vec[3].trim().parse::<isize>().unwrap();
                    }//end room ID check IF statement
                }
            }//end for j loop
        }//end for i loop

    }
    else {
        return (to_build, String::from("err: number of lines under #rooms header are off. Double check the room lines"));
    }

    //~~~~~~~fecthing item info~~~~~~~~~
    if num_items % item_dec_lines == 0 {
        //get the sub_arr for the item stuff
        let left_ind = items_ind + 1;
        let right_ind = items_end;
        items_vec.clone_from_slice(&lines[left_ind..right_ind + 1]);
        
        //get number of items
        let num_items: usize = items_vec.len() / item_dec_lines;
    }
    else {
        return (to_build, String::from("err: number of lines under #items header are off. Double check the item lines"));
    }

    build_msg.push_str("success");
    (to_build,build_msg)
}