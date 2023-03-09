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
    println!("Hello, world!");

    let built_world = 
        builder::build_world(String::from("C:\\Repos\\rusty-tork\\tork\\house.trksrc"));
    
    let world_name = &built_world.name;
    println!("The world name is: {world_name}");
}
