use std::io;
use std::io::Write;
use std::ptr;
use std::ptr::null;
use rand::Rng;

use crate::world::World;
use crate::world::State;
use crate::world::item::{Item, ItemType};
use crate::world::player;
use crate::world::room::{Directions};
use crate::world::room::Room;

pub enum Actions{
    Nvld, //invalid
    Quit,
    Move,
    Take,
    Drop,
    Atk,
    Inv,
    Look,
}

pub fn run<'a>(w: &'a mut World) -> (&'a World, State) {
    //initialize input and player
    let mut input = String::new();
    let mut in_dark: bool = false;
    let mut player_seen: bool = false;

    while input != "quit" {

        //~~~Announce Room~~~
        in_dark = announce_room(&w, holding_light(&w));

        //do the grue thing
        if w.grue_enabled && in_dark {
            println!("Oh, no!  A fearsome grue slithered into the room and devoured you.");
            return (w, State::DEATH);
        }
        
        announce_npcs(w);

        //~~~NPC Action~~~
        let mut npc_ind = 0;
        let mut found = false;
        for (i, npc) in w.npcs.iter().enumerate() {
            if npc.loc == w.player.loc{
                npc_ind = i;
                found = true;
            }
        }

        if found{
            if player_seen{
                let result = attack_player(w, w.npcs[npc_ind].id);
            println!("{}",result.1);
            }
            else {
                player_seen = true;
            }
        }
        else {
            player_seen = false;
        }

        //~~~Take Player Input~~~
        print!(">"); 
        io::stdout().flush().expect("Error writing to console buffer");

        input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        input = input.to_ascii_lowercase();
        input = String::from(input.trim_matches('\n').trim_matches('\r'));
        let words: Vec<&str> = input.split(' ').collect();
        
        let action = parse_action(&words[0]);

        //~~~Execute Action~~~
        match action{
            Actions::Move => {
                if words.len() > 1 {
                    let mut valid_dir = false;
                    let mut go = Directions::North;

                    match words[1]{
                        "north" | "n" => {
                            valid_dir = true;
                            go =  Directions::North;
                        },
                        "east" | "e" => {
                            valid_dir = true;
                            go =  Directions::East;
                        },
                        "south" | "s" => {
                            valid_dir = true;
                            go =  Directions::South;
                        },
                        "west" | "w" => {
                            valid_dir = true;
                            go =  Directions::West;
                        },
                        _ => println!("{} is not a valid direction", words[1]),
                    }

                    //move player
                    if valid_dir {
                        let result = move_player(w, go);
                        if !result.0 {
                            println!("{}",result.1);
                        }
                    }
                }
                else {
                    println!("Which way would you like to go?");
                }
            },
            Actions::Take => {
                //take item
                let rem_text = words[1..].join(" ");

                //search for item id by name
                let fetch_result = w.fetch_item_id(&rem_text);
                 if fetch_result.1 {
                    let take_result = take_item(w, fetch_result.0);
                    println!("{}", take_result.1);
                }
                else {
                    println!("You cannot take \"{}\"", rem_text);
                }
            },
            Actions::Drop => {
                let rem_text = words[1..].join(" ");

                //search for item id by name
                let result = w.fetch_item_id(&rem_text);
                if result.1 == true {
                    let result = drop_item(w, result.0);
                    if !result.0 {
                        println!("{}",result.1);
                    }
                }
                else {
                    println!("{} is not an interactable item", rem_text);
                }
            },
            Actions::Inv => {
                println!("You are holding:");
                //loop through the player's inventory
                for item in w.player.inv.iter() {
                    //if that slot is not "empty"
                    if !item.eq(&-1) {
                        //get the index of the item in the world's vector
                        let result = w.get_item_index(*item as usize);
                        if result.1 {
                            let world_ind: usize = result.0;
                            println!("{}", w.items[world_ind].name);
                        }
                        else {
                            println!("ERROR: Item id in Player inventory is not in the World's Item vector! wtf happened?");
                        }
                    }
                }
            },
            Actions::Atk => {
                //combat
                let rem_text = words[1..].join(" ");

                //search for npc by name
                let result = w.fetch_npc_id(&rem_text);
                if result.1 {
                    let result = attack_npc(w, result.0);
                    println!("{}",result.1);
                }
                else {
                    println!("You cannot attack \"{}\"", rem_text);
                }
            },
            Actions::Quit => {
                return (w, State::QUIT);
            },
            Actions::Nvld | _ => println!("The only commands I currently recognize are \"go\", \"walk\", \"take\", \"drop\"or \"quit\""),
        }        

        //assess player condition
        if w.player.hit_points == 0 {
            return (w, State::DEATH);
        }

        //empty line so the turns are easier to make out
        println!("");
    }

    (w, State::ERROR)
}

//returning true = player in darkness
pub fn announce_room(w: &World, light_held: bool) -> bool {
    unsafe {
        let curr_room: &Room = &(*w.player.loc);

        let room_name = &curr_room.name;
        let room_desc = &curr_room.desc;

        //basic announement
        println!("{}",room_name);
        println!("{}",room_desc);

        //announce pathways
        if (*curr_room).pathways[0].path != ptr::null_mut() {
            let pres_phrase = &curr_room.pathways[0].pres_phrase;
            let door_name = &curr_room.pathways[0].name; 
            println!("{} {} to the North",pres_phrase, door_name);
        }
        if (*curr_room).pathways[1].path != ptr::null_mut() {
            let pres_phrase = &curr_room.pathways[1].pres_phrase;
            let door_name = &curr_room.pathways[1].name; 
            println!("{} {} to the East",pres_phrase, door_name);
        }
        if (*curr_room).pathways[2].path != ptr::null_mut() {
            let pres_phrase = &curr_room.pathways[2].pres_phrase;
            let door_name = &curr_room.pathways[2].name; 
            println!("{} {} to the South",pres_phrase, door_name);
        }
        if (*curr_room).pathways[3].path != ptr::null_mut() {
            let pres_phrase = &curr_room.pathways[3].pres_phrase;
            let door_name = &curr_room.pathways[3].name; 
            println!("{} {} to the West",pres_phrase, door_name);
        }

        //announce items in room
        let mut items: Vec<&str> = Vec::new();

        for item in curr_room.floor.iter() {
            if !item.eq(&-1) {
                let item_id = item.clone() as usize;
                let world_ind = (*w).get_item_index(item_id).0;

                items.push((&(*w).items[world_ind].name).as_str());
            }
        }

        if items.len() > 0 {
            println!("The room contains:");
            for item in items.iter() {
                println!("{}",item);
            }
        }

        //announce darkness
        if curr_room.dark && !light_held {
            println!("It is pitch dark. You are likely to be eaten by a grue");
            return true;
        }
    }
    return false
}

pub fn announce_npcs(w: &mut World) {
    
    let mut npc_ind = 0;
    let mut found: bool = false;
    let mut npc_dead: bool = false;
    for (i, npc) in w.npcs.iter().enumerate() {
        if npc.loc == w.player.loc{
            if npc.health == 0 {
                npc_dead = true;
            }

            npc_ind = i;
            found = true;
        }
    }

    if found {
        if npc_dead {
            //announce NPC death
            println!("{} crumbles into dust",w.npcs[npc_ind].name);
            w.npcs[npc_ind].loc = null();
        }
        else {
            println!("There is a {}", w.npcs[npc_ind].name);
        }
    }
}

pub fn attack_npc(w: &mut World, npc_id: usize) -> (bool, String) {
    let result = w.get_npc_index(npc_id);
    if result.1 {
        let npc_ind = result.0;
        
        if w.npcs[npc_ind].loc == w.player.loc {
            let result = w.find_first_item_type(ItemType::WPN);
            if result.1 {
                let first_wpn_ind = result.0;

                //get weapon values
                let wpn_dmg = w.items[first_wpn_ind].val1.clone() as u16;
                let hit_chance = w.items[first_wpn_ind].val2.clone() as u16 ;
            
                //roll for hit
                let roll = rand::thread_rng().gen_range(1..=10000);
                if roll <= hit_chance {
                    //successful hit
                    if w.npcs[npc_ind].health <= wpn_dmg {
                        w.npcs[npc_ind].health = 0;
                    }
                    else {
                        w.npcs[npc_ind].health -= wpn_dmg;
                    }
                    let mut return_msg = String::from("You hit the ");
                    return_msg.push_str(&w.npcs[npc_ind].name);
                    return (true, return_msg);
                }
                else {
                    let mut return_msg = String::from("You miss the ");
                    return_msg.push_str(&w.npcs[npc_ind].name);
                    return (true, return_msg);
                }
            }
        }
        else {
            //target npc is not in the room w/ player
            let mut return_msg = String::from("You do not see ");
            return_msg.push_str(&w.npcs[npc_ind].name.as_str());
            return (false, return_msg)
        }
    }
    else{
        return (false, String::from("err: bad npc_id passed to attack_npc"))
    }

    (false, String::from("err: Unexpected error in attack_npc"))
}

pub fn attack_player(w: &mut World, npc_id: usize) -> (bool, String) {
    let result = w.get_npc_index(npc_id);
    if result.1 {
        let npc_ind = result.0;

        //assume npc is in the same room as player; this should be checked already
        let npc_dmg = w.npcs[npc_ind].hit_pwr;
        let npc_hit_chance = w.npcs[npc_ind].hit_chance;

        let roll = rand::thread_rng().gen_range(1..=10000);
        if roll <= npc_hit_chance {
            //npc hit player
            if w.player.hit_points <= npc_dmg {
                w.player.hit_points = 0;
            }
            else {
                w.player.hit_points -= npc_dmg;
            }

            let mut return_msg = String::from(&w.npcs[npc_ind].name);
            return_msg.push_str(" hit you");
            return (true, return_msg);
        }
        else {
            let mut return_msg = String::from(&w.npcs[npc_ind].name);
            return_msg.push_str(" misses you");
            return (true, return_msg);
        }
    }

    (false, String::from("err: Unexpected error in attack_player"))
}

pub fn has_key(w: &World, lock: usize) -> bool {
    if lock == 0{
        return true;    
    }

    for item in w.player.inv.iter() {
        //check if there is an item in the inv slot
        if !item.eq(&-1) {
            //get the index for that item in the World's Item arr
            let item_ind = (*w).get_item_index((*item) as usize).0;
            let world_item = &(*w).items[item_ind];
            //get if the item is a key
            if world_item.item_type == ItemType::KEY && 
                world_item.val1 == lock{
                return true;
            }
        }
    }

    return false
}

pub fn holding_light(w: &World) -> bool {
    //loop through player inv
    for item in w.player.inv.iter() {
        //check if inv slot is not empty
        if !item.eq(&-1) {
            let to_check: &Item = &(*w).items[(*item as usize)];
            if to_check.item_type == ItemType::LIT {
                return true;
            }
        }
    }

    false
}

pub fn take_item(w: &mut World, item_id: usize) -> (bool, String) {
    unsafe {
        //get item's index of world vector
        let world_ind: usize = w.get_item_index(item_id).0;

        //get the room the player is current in
        let room_ind: usize = w.get_room_index((*w.player.loc).id).0; //wtf is this syntax
        let curr_room: &mut Room = &mut (*w).rooms[room_ind];

        //check if player inv is full
        if w.player.count_inv() <= w.player.inv_lim {
            //add item to player inv
            if w.player.pocket_item(item_id) {
                //remove item from floor
                if curr_room.handoff_item(item_id) {
                    (*w).items[world_ind].loc = -1;
                    let mut return_msg = String::from("You have taken ");
                    return_msg.push_str((*w).items[world_ind].name.as_str());
                    return (true, return_msg);
                }
            }
            else {
                return (false, String::from("err: item could not be added to player inv"));
            }
        }
        else {
            return (false, String::from("You cannot carry another item"));
        }
    }

    return (false, String::from("err: unexpected error in take_item()"))
}

pub fn drop_item(w: &mut World, item_id: usize) -> (bool, String) {
    unsafe {
        let world_ind: usize;
        let play_ind: usize;
        let room_ind: usize = w.get_room_index((*(w.player).loc).id).0; //wtf is this syntax

        //get item's index of world vector
        world_ind = w.get_item_index(item_id).0;
        //get item's index for player inventory
        let result = w.player.get_inv_ind(item_id);
        play_ind = result.0;

        let curr_room: &mut Room = &mut (*w).rooms[room_ind];

        //check if floor has space
        if !(*curr_room).floor_full() {
            //check if the item was found in player inv
            if result.1 {
                //set player.inv[n] to -1 (that slot is now "empty")
                w.player.drop_item(play_ind);
                
                //set item's location
                w.items[world_ind].loc = (*curr_room).id as isize;

                //add item to room's floor
                (*curr_room).catch_item(item_id);

                return (true, String::from(""));
            }
            else {
                return (false, String::from("You do not have that in your inventory."));
            }
        }
    }

    (false, String::from("There is not enough space on the floor."))
}

pub fn parse_action(input: &str) -> Actions {
    let action: Actions;

    match input {
        "attack" | "kill" => action = Actions::Atk,
        "drop" => action = Actions::Drop,
        "inv" | "i" | "inventory" => action = Actions::Inv,
        "look" => action = Actions::Look,
        "move" | "go" | "walk" => action = Actions::Move,
        "take" => action = Actions::Take,
        "quit" => action = Actions::Quit,
        _ => action = Actions::Nvld,
    }

    action
}

pub fn move_player(w: &mut World, dir: Directions) -> (bool, String) {
    unsafe {
        //get the next room the player wants to go to
        let next_door = &(*w.player.loc).pathways[dir as usize];
        let next_room = next_door.path;

        //make sure that direction isn't NULL
        if next_room != ptr::null_mut() {
            
            //make sure door isn't locked
            let mobile: bool;

            //check if door is locked and if player has the key
            if next_door.lock.eq(&0) || has_key(w, next_door.lock) {
                mobile = true;
            }
            else{
                mobile = false;
            }

            if mobile {
                //change player's location
                w.player.loc = next_room;
                return (true, String::new());
            }
            else{
                return (false, String::from("The door is locked"));
            }
        }
        else {

            return (false, String::from("You can't go that way."));
        }
    }
    //return (false, String::from("err: an unexpected error has occurred in move_player()"));
}