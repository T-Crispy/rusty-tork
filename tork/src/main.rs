use std::{io, env};
//use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::fs::OpenOptions;

use crate::world::World;
use crate::world::player::Player;

pub mod builder;
pub mod driver;
pub mod world;

//flag go back and change all the tuple returning functions to use the Option type instead

fn main() {
    let dir: PathBuf = env::current_dir().unwrap();
    let mut running: bool = true;
    let ver = "v0.2.7.0";

    println!("Welcome to Tork {}!", ver);
 
    while running{
        let mut file_name = String::new();

        //prompt for file name
        //io::stdout().lock().write_all(b"Enter source file name (w/o extension): ");
        print!("Enter source file name (w/o extension): ");
        io::stdout().flush().expect("");
        
        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read line");

        let mut path = String::from(dir.as_os_str().to_str().unwrap());
        path.push_str("\\");
        path.push_str(file_name.as_str());
        path = String::from(path.trim_matches('\n'));
        path = String::from(path.trim_matches('\r'));
        path.push_str(".trksrc");

        let file = OpenOptions::new().read(true).open(&path);
        if file.is_ok() {
            println!("Building World..");
            let result = builder::build_world(path);
            
            if result.1.contains("success") {
                if result.1.contains("warn"){
                    println!("{}",result.1.trim_end_matches("\nsuccess"));
                }

                let mut built_world = result.0;
                let mut player: Player = Player {loc: &built_world.rooms[0], inv_lim: 7, inv: [-1;7], hit_points: 100};
                //initialize item locations
                for item in built_world.items.iter() {
                    if item.loc.eq(&-1){
                        //add to player inventory
                        if !player.pocket_item(item.id) {
                            if built_world.rooms[0].catch_item(item.id) {
                                println!("warn: more than 7 items have been declared to start in the player's inventory. Placing overlfow item into first room.");
                            }
                            else {
                                println!("warn: too many overflow items for player inventory; item #{} will be living in the void", item.id);
                            }
                        }
                    }
                    else if item.loc.is_positive() {
                        //add the item to its room floor
                        
                        //get the room index
                        let room_ind = built_world.get_room_index(item.loc as usize).0;
                        let curr_room = &mut built_world.rooms[room_ind];

                        if !curr_room.catch_item(item.id) {
                            println!("warn: No space in room #{} for item #{}",curr_room.id, item.id);
                        }
                    }
                }

                println!("Starting {}...\n", &built_world.name);
                let result: (&World, bool) = driver::run(&mut built_world, &mut player);
                if !result.1 {
                    print!("An Error was encountered while running the world");
                }
            }
            else {
                println!("There was an error while building\n{}",result.1);
            }

            println!("Would you like to load another world? (Y/N)");

            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input.contains('n') || input.contains('N') {
                running = false;
            }
        }
        else {
            print!("Filename is not good :(\n");
        }
    }

    print!("Tork has finished running. Press Any Key to close Console Window.");
    io::stdout().flush().expect("");
    let mut res = String::from("");
    io::stdin()
        .read_line(&mut res)
        .expect("");
    
}
