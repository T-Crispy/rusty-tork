use crate::world::World;
use crate::world::room::Room;

//use core::num;
use std::fs;
use std::ptr;

pub fn build_world(filename: String) -> (World, String) {
    let mut build_msg = String::from("");
    let room_dec_lines: usize = 4;
    let headers: Vec<&str> = vec!["#world","#rooms"];

    let mut to_build = World{name: String::from("new world"), rooms: Vec::new()};
    
    //let mut finished = false;
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();

    let mut world_ind: isize = -1;

    let mut rooms_ind: isize = -1;
    let mut rooms_end: usize = 0;

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
        i += 1;
    } //end header check

    if world_ind == -1 {
        return (to_build, String::from("err: #world header missing"));
    }
    if rooms_ind == -1 {
        return (to_build, String::from("err: #rooms header missing"));
    }

    let rooms_ind: usize = rooms_ind as usize;
    let mut rooms_vec = vec![""; rooms_end - rooms_ind];
    let num_rooms: usize = rooms_end - rooms_ind;

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
        let null_room: Room = Room {
            name: String::from(""),
            desc: String::from(""),
            id: 0,
            pathways: [ptr::null_mut(), 
                        ptr::null_mut(), 
                        ptr::null_mut(),
                        ptr::null_mut()]
        };
        to_build.rooms = vec![null_room; num_rooms];

        let mut paths_vec = vec![""; num_rooms];

        //loop through the room lines to get the id,name, & desc of each room
        for i in 0..num_rooms{
            let offset: usize = i * room_dec_lines;

            //get room ID
            if rooms_vec[offset].starts_with('#') {
                let id_parse_result = rooms_vec[offset].trim().trim_start_matches('#').parse::<usize>();
                if id_parse_result.is_err() {
                    let mut err_msg = String::from("err: Could not parse the ID number for room");
                    err_msg.push_str(offset.to_string().as_str());
                    return (to_build, err_msg);
                }
                to_build.rooms[i].id = id_parse_result.unwrap();
            }
            
            //get room name and desc
            to_build.rooms[i].name = rooms_vec[offset + 1].to_string();
            to_build.rooms[i].desc = rooms_vec[offset + 2].to_string();

            //get room pathways
            paths_vec[i] = rooms_vec[offset + 3].trim();
        }//end for i loop

        //loop through rooms to set pathways
        for i in 0..num_rooms{
            let paths: Vec<&str> = paths_vec[i].split('|').collect();

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
                    let get_room_result: (usize, bool) = to_build.get_room_index(num_id);
                    if !get_room_result.1 {
                        let mut warn_msg = String::from("warn: a path way for room #");
                        warn_msg.push_str(to_build.rooms[i].id.to_string().as_str());
                        warn_msg.push_str(" could not be linked to its defined room\n");
                        build_msg.push_str(warn_msg.as_str());
                    }
                    else {
                        let room_id: usize = get_room_result.0;
                    to_build.rooms[i].pathways[j] = &mut to_build.rooms[room_id];
                    }
                }
            }//end for j loop
        }

    }
    else {
        return (to_build, String::from("err: number of lines between #rooms and #paths headers are off. Double check the room lines"));
    }

    build_msg.push_str("success");
    (to_build,build_msg)
}