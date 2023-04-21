Tork has two main parts: The Builder and the Driver.

For The Builder, users only need to know how to format the source files correctly, so their worlds can be properly built.
This information can be found in the Source Format Overview text file.

For The Driver, users need to know the current commands, which are:
    -"move" / "go" / "walk": commands player to move in a cardinal direction
        > format: "move {direction}"
        > The current directions accepted are North, East, South, and West
    -"take" / "drop": takes or drops the item with the specified name
        > format: "{take/drop} {item_name}"
        > When the room is announced, the player will be told "The room contains:.." where each line after is the name of an item in that room
    -"inv" / "i" / "inventory": prints all the items in the players inventory.
        > this command requires nothing after the command word itself
        > (the current inventory limit is 7)
    -"attack" / "kill": attacks a given enemy with the first item in the player's inventory
        > format: "{attack/kill} {npc name}"
        > Tork (currently) will use the first wpn in inventory for the attack 
    -"quit": ends the current game, exiting The Driver

    ("look" is currently parsed, but nothing it done with them yet)