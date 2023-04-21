use crate::world::World;
use crate::world::item::Item;
use crate::world::item::ItemType;
use crate::world::npc::NPC;
use crate::world::room::Doorway;
use crate::world::room::Room;
use crate::world::player::Player;

//use core::num;
use std::fs;
use std::ptr;
use std::ptr::null;

pub fn build_world(filename: String) -> (World, String) {
    let mut build_msg = String::from("");
 
    let world_dec_lines: usize = 3;

    let room_dec_lines: usize = 8;
    let num_paths: usize = 4;
    let north_offset = 3;
    let east_offset = 4;
    let south_offset = 5;
    let west_offset = 6;

    let item_dec_lines: usize = 5;

    let npc_dec_lines: usize = 6;

    let headers: Vec<&str> = vec!["#world","#rooms", "#items", "#npcs"];

    let mut to_build = World{name: String::from("new world"), 
                            rooms: Vec::new(), 
                            items: Vec::new(),
                            npcs: Vec::new(),
                            grue_enabled: false,
                            player: Player{ loc: null(),
                                            inv: [-1; 7],
                                            inv_lim: 7,
                                            hit_points: 100}};
    
    //let mut finished = false;
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\r\n").collect();

    let mut world_ind: isize = -1;
    let mut world_end: usize = 0;

    let mut rooms_ind: isize = -1;
    let mut rooms_end: usize = 0;

    let mut items_ind: isize = -1;
    let mut items_end: usize = 0;

    let mut npcs_ind: isize = -1;
    let mut npcs_end: usize = 0;

    //check for headers
    let mut i: usize = 0;
    //for mut i in 0..lines.len()
    while i < lines.len() {
        //remove any existing carriage return
        let mut curr_line = lines[i];

        //check for headers
        match curr_line {
            "#world" => {
                world_ind = i as isize;

                i += 1;
                curr_line = lines[i];

                while !(headers.contains(&curr_line)) && i < lines.len(){
                    curr_line = lines[i];
                    i += 1;
                }

                if i != lines.len() {
                    i -= 1;
                }
            
                world_end = i;
                i -= 1;
            },
            "#rooms" => {
                rooms_ind = i as isize;

                i += 1;
                curr_line = lines[i];
    
                while !(headers.contains(&curr_line)) && i < lines.len(){
                    //continue iterating until the next header is reached or the end of line
                    curr_line = lines[i];
                    i += 1;
                }
    
                if i != lines.len() {
                    i -= 1;
                }
                rooms_end = i;
                i -= 1;                
            },
            "#items" => {
                items_ind = i as isize;

                i += 1;
                curr_line = lines[i];
    
                while !(headers.contains(&curr_line)) && i < lines.len() {
                    //continue iterating until the next header
                    curr_line = lines[i];
                    i += 1;
                }
    
                if i != lines.len() {
                    i -= 1;
                }
                items_end = i;
                i -= 1;                
            },
            "#npcs" => {
                npcs_ind = i as isize;

                i += 1;
                curr_line = lines[i];

                while !(headers.contains(&curr_line)) && i < lines.len() {
                    //continue iterating until the next header
                    curr_line = lines[i];
                    i += 1;
                }

                if i != lines.len() {
                    i -= 1;
                }
                npcs_end = i;
                i -= 1;
            },
            _ => {}
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
    if npcs_ind == -1{
        return (to_build, String::from("err: #npcs header missing"));
    }

    
    let world_ind: usize = world_ind as usize;
    let num_world_lines: usize = world_end - world_ind - 1;
    let mut world_vec = vec![""; num_world_lines];
    let mut start_loc_id: usize = 0;

    let rooms_ind: usize = rooms_ind as usize;
    let num_rooms: usize = rooms_end - rooms_ind - 1;
    let mut rooms_vec = vec![""; num_rooms];

    let items_ind: usize = items_ind as usize;
    let num_items: usize = items_end - items_ind - 1;
    let mut items_vec = vec![""; num_items];

    let npcs_ind: usize = npcs_ind as usize;
    let num_npcs: usize = npcs_end - npcs_ind - 1;
    let mut npcs_vec = vec![""; num_npcs];

    //~~~~~~~fecthing world info~~~~~~~~~
    if num_world_lines % world_dec_lines == 0 {
        //get subarr of lines under the world header
        let left_ind = world_ind + 1;
        let right_ind: usize = world_end;

        world_vec.clone_from_slice(&lines[left_ind..right_ind]);

        to_build.name = world_vec[0].to_string();
        
        match world_vec[1] {
            "GRUE" => { to_build.grue_enabled = true; },
            "NO GRUE" => { to_build.grue_enabled = false; },
            _ => {} //flag add in an error message
        }

        //get starting location
        if world_vec[2].starts_with("~start") {
            let result = world_vec[2].trim_start_matches("~start").trim().parse::<usize>();
            if result.is_err() {
                return (to_build, String::from("err: Could not parse the ID number for start location"));
            }
            start_loc_id = result.unwrap();
        }
    }

    //~~~~~~~fecthing room info~~~~~~~~~
    //make sure the # of lines between the headers = 3r + 1
    //wher r is the # of rooms
    if num_rooms % room_dec_lines == 0 {
        //get the sub_arr for the room and path stuff
        let left_ind = rooms_ind+1;
        let right_ind = rooms_end;
        rooms_vec.clone_from_slice(&lines[left_ind..right_ind]);

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
            dark: false,
            floor: [-1;7]
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

                let destination = door_vec[4].trim();
                if destination != "NULL"{
                    let num_id: usize = destination.parse::<usize>().unwrap();
                    
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
                        to_build.rooms[i].pathways[dir_ind].lock = door_vec[3].trim().parse::<usize>().unwrap();
                    }//end room ID check IF statement
                }
            }//end for j loop
        }//end for i loop


        //set the starting location for the world
        let result = to_build.get_room_index(start_loc_id);
        if result.1 {
            let start_loc_ind = result.0;
            to_build.player.loc = &to_build.rooms[start_loc_ind];
        }
        else {
            let mut err_msg = String::from("err: could not find a room with ID: ");
            err_msg.push_str(start_loc_id.to_string().as_str());
            err_msg.push_str(" for starting location");
            return (to_build,err_msg);
        }
    }
    else {
        return (to_build, String::from("err: number of lines under #rooms header are off. Double check the room lines"));
    }

    //~~~~~~~fecthing item info~~~~~~~~~
    if num_items % item_dec_lines == 0 {
        //get the sub_arr for the item stuff
        let left_ind = items_ind + 1;
        let right_ind = items_end;
        items_vec.clone_from_slice(&lines[left_ind..right_ind]);
        
        //get number of items
        let num_items: usize = items_vec.len() / item_dec_lines;

        //initialize to_build's items
        let null_item: Item = Item{
            item_type: ItemType::KEY,
            id: 0,
            name: String::from(""),
            val1: 0,
            val2: 0,
            loc: -1,
        };
        to_build.items = vec![null_item; num_items];

        //loop through the item lines
        for i in 0..num_items{
            let offset: usize = i * item_dec_lines;

            //get item ID
            if items_vec[offset].starts_with('#') {
                let id_parse_result = items_vec[offset].trim().trim_start_matches('#').parse::<usize>();
                if id_parse_result.is_err(){
                    let mut err_msg = String::from("err: Could not parse the ID number for item ");
                    err_msg.push_str(offset.to_string().as_str());
                    return (to_build, err_msg);
                }
                to_build.items[i].id = id_parse_result.unwrap();
            }

            let item_type = items_vec[offset + 1].trim();
            if item_type == "WPN" {
                to_build.items[i].item_type = ItemType::WPN;
            }
            else if item_type == "LIT" {
                to_build.items[i].item_type = ItemType::LIT;
            }

            //get the items starting location
            if items_vec[offset + 2].starts_with("~loc") {
                let id_parse_result = items_vec[offset + 2].trim_start_matches("~loc").trim().trim_start_matches("~loc").parse::<isize>();
                if id_parse_result.is_err(){
                    let mut err_msg = String::from("err: Could not parse the Room ID number for item ");
                    err_msg.push_str(offset.to_string().as_str());
                    return (to_build, err_msg);
                }
                to_build.items[i].loc = id_parse_result.unwrap();
            }
            else{
                //flag add in error message
            }

            //get item name
            to_build.items[i].name = items_vec[offset + 3].to_string();

            let vals_vec: Vec<&str> = items_vec[offset + 4].split('|').collect();

            if vals_vec.len() ==  2{
                //get val1
                let parse_result = vals_vec[0].trim().parse::<usize>();
                if parse_result.is_err() {
                    //flag return error message
                }
                to_build.items[i].val1 = parse_result.unwrap();

                //get val 2
                let parse_result = vals_vec[1].trim().parse::<usize>();
                if parse_result.is_err() {
                    //flag return error message
                }
                to_build.items[i].val2 = parse_result.unwrap();
            }
            
        }

    }
    else {
        return (to_build, String::from("err: number of lines under #items header are off. Double check the item lines"));
    }

    //fetching npc information
    if num_npcs % npc_dec_lines == 0 {
        //get the sub_arr for the npc stuff
        let left_ind = npcs_ind + 1;
        let right_ind = npcs_end;
        npcs_vec.clone_from_slice(&lines[left_ind..right_ind]);

        //get the number of npcs
        let num_npcs: usize = npcs_vec.len() / npc_dec_lines;

        //initialize to_build's npc`s
        let null_npc: NPC = NPC { id: 0, 
                                loc: null(), 
                                name: String::from(""),
                                health: 0, 
                                hit_pwr: 0, 
                                hit_chance: 0 };
        to_build.npcs = vec![null_npc; num_npcs];
        
        for i in 0..num_npcs {
            let offset: usize = i * npc_dec_lines;

            //get npc ID
            if npcs_vec[offset].starts_with('#') {
                let id_parse_result = npcs_vec[offset].trim_start_matches('#').parse::<usize>();
                if id_parse_result.is_err() {
                    let mut err_msg = String::from("err: Could not parse the ID number for NPC ");
                    err_msg.push_str(offset.to_string().as_str());
                    return (to_build, err_msg);
                }
                to_build.npcs[i].id = id_parse_result.unwrap();
            }
            else {
                //flag add error message
            }

            //get the NPCs starting location
            if npcs_vec[offset + 1].starts_with("~loc") {
                let id_parse_result = npcs_vec[offset + 1].trim_start_matches("~loc").trim().parse::<usize>();
                if id_parse_result.is_err() {
                    let mut err_msg = String::from("err: Could not parse the Room ID number for NPC ");
                    err_msg.push_str(offset.to_string().as_str());
                    return (to_build, err_msg);
                }

                //get the starting location of the NPC
                let result = to_build.get_room_index(id_parse_result.unwrap());
                if result.1 {
                    let start_loc_ind = result.0;
                    to_build.npcs[i].loc = &to_build.rooms[start_loc_ind];
                }
                else {
                    let mut err_msg = String::from("err: could not find a room with ID: ");
                    err_msg.push_str(start_loc_id.to_string().as_str());
                    err_msg.push_str(" for NPC #");
                    err_msg.push_str(to_build.npcs[i].id.to_string().as_str());
                    err_msg.push_str("\'s starting location");
                    return (to_build,err_msg);
                }
            }
            else {
                //flag add error message
            }

            //get npc name
            to_build.npcs[i].name = npcs_vec[offset + 2].to_string();

            //get npc hp
            let parse_result = npcs_vec[offset + 3].parse::<u16>();
            if parse_result.is_err() {
                let mut err_msg = String::from("err: could not parse the Hit Points for NPC #");
                err_msg.push_str(to_build.npcs[i].id.to_string().as_str());
                err_msg.push_str(". Make sure the max value of 65,535 has not been exceeded");
                return (to_build,err_msg);
            }
            else {
                to_build.npcs[i].health = parse_result.unwrap();
            }

            //get npc attack dmg
            let parse_result = npcs_vec[offset + 4].parse::<u16>();
            if parse_result.is_err() {
                let mut err_msg = String::from("err: could not parse the Attack Damage for NPC #");
                err_msg.push_str(to_build.npcs[i].id.to_string().as_str());
                err_msg.push_str(". Make sure the max value of 65,535 has not been exceeded");
                return (to_build,err_msg);
            }
            else {
                to_build.npcs[i].hit_pwr = parse_result.unwrap();
            }

            //get npc hit chance
            let parse_result = npcs_vec[offset + 5].parse::<u16>();
            if parse_result.is_err() {
                let mut err_msg = String::from("err: could not parse the Hit-chance for NPC #");
                err_msg.push_str(to_build.npcs[i].id.to_string().as_str());
                err_msg.push_str(". Make sure the max value of 65,535 has not been exceeded");
                return (to_build,err_msg);
            }
            else {
                to_build.npcs[i].hit_chance = parse_result.unwrap();
            }

        }//end for i loop
    }
    else{
        return (to_build, String::from("err: number of lines under #npcs header are off. Double check the npc lines"));
    }

    build_msg.push_str("success");
    (to_build,build_msg)
}