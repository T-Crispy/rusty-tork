use std::env;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::fs::OpenOptions;

pub mod builder;
pub mod driver;
pub mod world;

/*
___make builder capable to build source file
    >rooms will only have NESW directions
___make driver capable of playing through a world
    >just walking between rooms and outputing the name & desc of the room
*/

fn main() {
    let dir: PathBuf = env::current_dir().unwrap();
    let mut running: bool = true;
    
 
    while running{
        let mut file_name = String::new();

        //prompt for file name
        //io::stdout().lock().write_all(b"Enter source file name (w/o extension): ");
        print!("Enter source file name (w/o extension): ");
        io::stdout().flush();
        
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
            let built_world = 
                builder::build_world(path);
            
            //call driver

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
            print!("Filename is not good :(");
        }
    }

    
    
    
}
