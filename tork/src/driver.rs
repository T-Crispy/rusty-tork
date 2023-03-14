use std::io;
use std::io::Write;
use std::ptr;

use crate::world::World;
use crate::world::room::{Directions};
use crate::world::character::Character;

pub fn run(w: &mut World) -> (&World, bool) {
    //initialize input and player
    let mut input = String::new();
    let mut player: Character = Character {loc: &w.rooms[0] };

    while input != "quit" {
        unsafe{
            let room_name = &(*player.loc).name;
            let room_desc = &(*player.loc).desc;

            println!("{}",room_name);
            println!("{}",room_desc);
        }

        //println!("{}",(*player.loc).name);
        //println!("{}",(*player.loc).desc);
        //prompt user for input
        print!(">"); 
        io::stdout().flush().expect("Error writing to console buffer");

        input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        input = input.to_ascii_lowercase();
        input = String::from(input.trim_matches('\n').trim_matches('\r'));
        let words: Vec<&str> = input.split(' ').collect();
        
        if words[0] == "go" || words[0] == "walk" {
            
            //make sure there are at least 2 words
            if words.len() > 1 {
                let mut go: Directions = Directions::North;
                let mut valid_dir: bool = true;

                //get the direction the player 
                if words[1] == "north" {
                    go = Directions::North;
                }
                else if words[1] == "east" {
                    go = Directions::East;
                }
                else if words[1] == "south" {
                    go = Directions::South;
                }
                else if words[1] == "west" {
                    go = Directions::West;
                }
                else {
                    valid_dir = false;
                }

                //move the player
                if valid_dir{
                    if !move_player(&mut player, go) {
                        println!("You can't go that way!");
                    }
                }
                else {
                    println!("{} is not a valid direction",words[1]);
                }
            }
            else {
                println!("You must enter a direction to go (North, South, East, or West)");
            }
        }
        else if words[0] == "quit"{
            return (w, true);
        }
        else {
            println!("The only commands I currently recognize are \"go\", \"walk\", or \"quit\"");
        }
    }

    (w, false)
}

fn move_player(player: &mut Character, dir: Directions) -> bool {
    unsafe {
        //get the next room the player wants to go to
        let next_room = (*player.loc).pathways[dir as usize];

        //make sure that direction isn't NULL
        if next_room != ptr::null_mut() {
            //get the index of that room

            player.loc = next_room;
            return true;
        }
    }
    false
}