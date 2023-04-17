use std::io;
use std::io::Write;
use std::ptr;

use crate::world::World;
use crate::world::room::{Directions};
use crate::world::room::Room;
use crate::world::character::Character;

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

pub fn run(w: &mut World) -> (&World, bool) {
    //initialize input and player
    let mut input = String::new();
    let mut player: Character = Character {loc: &w.rooms[0], inv_lim: 7, inv: [-2;7]};

    while input != "quit" {

        //~~~Announce Room~~~
        unsafe{
            announce_room(&(*player.loc));
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
                        if !move_player(&mut player, go) {
                            println!("You can't go that way!");
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
                let result = w.fetch_item_id(&rem_text);
                if result.1 == true {
                    drop_item(&mut player, w, result.0);
                }
                else {
                    println!("{}", rem_text);
                }
            },
            Actions::Drop => {
                let rem_text = words[1..].join(" ");

                //search for item id by name
                let result = w.fetch_item_id(&rem_text);
                if result.1 == true {
                    let result = drop_item(&mut player, w, result.0);
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
                for item in player.inv.iter() {
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
                //add combat stuff here
            },
            Actions::Quit => {
                return (w, true);
            },
            Actions::Nvld | _ => println!("The only commands I currently recognize are \"go\", \"walk\", \"take\", \"drop\"or \"quit\""),
        }

        //~~~NPC Action~~~
        //flag to add
    }

    (w, false)
}

fn drop_item(p: &mut Character, w: &mut World, item_id: usize) -> (bool, String) {
    let world_ind: usize;
    let play_ind: usize;

    //get item's index of world vector
    world_ind = w.get_item_index(item_id).0;
    //get item's index for player inventory
    let result = p.get_inv_ind(item_id);
    play_ind = result.0;

    //add item to room's floor
    if true {
        if result.1 {
            //set player.inv[n] to -1 (that slot is now "empty")
            p.inv[play_ind] = -1;
            unsafe {
                //set item's location
                w.items[world_ind].loc = (*p.loc).id as isize;
            }
            return (true, String::from(""));
        }
        else {
            return (false, String::from("You do not have that in your inventory."));
        }
    }

    (false, String::from("There is not enough space on the floor."))
}

fn parse_action(input: &str) -> Actions {
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

fn announce_room(curr_room: &Room){
    let room_name = &curr_room.name;
    let room_desc = &curr_room.desc;

    println!("{}",room_name);
    println!("{}",room_desc);

    if (*curr_room).pathways[0].path != ptr::null_mut() {
        let pres_phrase = &curr_room.pathways[0].pres_phrase;
        let door_name = &curr_room.pathways[0].name; 
        println!("{} {} to the North",pres_phrase, door_name);
    }
    else if (*curr_room).pathways[1].path != ptr::null_mut() {
        let pres_phrase = &curr_room.pathways[1].pres_phrase;
        let door_name = &curr_room.pathways[1].name; 
        println!("{} {} to the East",pres_phrase, door_name);
    }
    else if (*curr_room).pathways[2].path != ptr::null_mut() {
        let pres_phrase = &curr_room.pathways[2].pres_phrase;
        let door_name = &curr_room.pathways[2].name; 
        println!("{} {} to the South",pres_phrase, door_name);
    }
    else if (*curr_room).pathways[3].path != ptr::null_mut() {
        let pres_phrase = &curr_room.pathways[3].pres_phrase;
        let door_name = &curr_room.pathways[3].name; 
        println!("{} {} to the West",pres_phrase, door_name);
    }
}

fn move_player(player: &mut Character, dir: Directions) -> bool {
    unsafe {
        //get the next room the player wants to go to
        let next_room = (*player.loc).pathways[dir as usize].path;

        //make sure that direction isn't NULL
        if next_room != ptr::null_mut() {
            //get the index of that room

            player.loc = next_room;
            return true;
        }
    }
    false
}