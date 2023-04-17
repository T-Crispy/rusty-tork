# Rusty Tork

## Current Stable Version: v0.2.7.0

### Project Description

This project for an Interactive Fiction game engine called “Tork”.
Interactive Fictions (IFs) are text based games that allow a player to type an input in order to interact with the world.
One IF that gained quite a bit of popularity at one point is the Zork series.
The aim of Tork will be to allow users to create their own “Zork” game.

### Getting Started - Compiling project files

#### Installing Rust

For compiling the project files in this repo, first you will need to install Rust. There are instructions for installing Rust here: <https://doc.rust-lang.org/book/ch01-01-installation.html>

I am using VS Code as my IDE for this project, so any further instructions will be for using VS Code.

#### VS Code Extensions

- C/C++ for Visual Studio Code
  - Extension ID: ms-vscode.cpptools
  - Purpose: for being able to use breakpoints and debug the code
- rust-analyzer
  - Extension ID: rust-lang.rust-analyzer
  - Purpose: double checking syntax, runs "cargo check" when you press "ctrl+s". This extension can get kind of annoying though with the text it inserts for display, so you may have to tweak the settings to fit your preferences.
- IntelliCode
  - Extension ID: VisualStudioExptTeam.vscodeintellicode
  - Purpose: not really needed, but I like to use it to save 0.5s of typing for variable names
- markdownlint
  - Extension ID: DavidAnson.vscode-markdownlint
  - Purpose: (optional) to help with writing the md files for patchnotes and such

> ### Most Recent Patch Notes (as of 04/17/23)
>
> #### Changes
>
> - Simple items the player can "take" and "drop"
>   - Simple Items consist of the following Item Types: weapons, light sources, keys
> - The player can now use the actions "take" and "drop" for items
> - The player now has an inventory to hold items, with a limit of 7 items
> - Rooms have a “isLit” flag. If this flag is false, then the room is considered dark.
>   - As a homage to Zork, if the player enters a dark room, the message “It is pitch black. You are likely to be eaten by a grue”. If the player stays in the dark room, then the player will be eaten by said grue.
> - Doors have been added to the rooms that can be locked
> - Doors have a presence-phrase, a name, a lock integer and a pointer to another room
>   - The presence-phrase is for when the driver announces the room. Instead of the possible directions being in the Room's description, that will be handled by the driver
>     - ex. For NORTH door named "chained gate" /w a pres-phrase of "There is a", after the Room's description the Driver would output
>     - \>"There is a chained gate to the NORTH"
>   - To enter a locked "door", the door's lock number and the key's value must match. A value of 0 is considrered unlocked
>   - Just like if a direction is set to NULL, if a door is locked the player will remain in the current room
> - Updates to the Builder:
> - A new section "#items" is now expected to be seen
>   - There does not have to be any defined items, but the items header must always be present
> - Each direction for a room is on its own line. These lines hold the information for its respective door
> - The Proto-SourceFormatOverview has been updated to reflect what the Builder now expects
> - The Fort.trksrc example file has been updated as well
>
> #### Bug Fixes
>
> - xx

### Version Control

#### Alpha Numbering

Alpha & Beta version numbering will be some number 0.X.Y.Z, where

- 'X' is the current phase number being worked on
- 'Y' is the last part of the current phase that has been completed
- 'Z' is for incremental changes before a part is complete

#### Post-Alpha Numbering

After version 1.0 has been reached, the version will be some number W.X.Y.Z, where

- 'W' is for major changes to the code structure or enough features have been added to warrant a change
- 'X' is for new features being added to the program
- 'Y' is for minor changes being made
- 'Z' is for bug fixes

### Phase Plan

01. ~~Simple Worlds~~
    1. ~~Get the Builder to be able build a world from a source file. For this phase, the rooms will only have NWSE directions.~~
    2. ~~Get the Driver to be able to play through a simple world, mostly just walking between rooms.~~
02. Simple Items
    1. ~~Add simple items the player can "take" and "drop"~~
    2. ~~Create item types: weapon, light source, keys~~
    3. ~~Update Rooms to have a “isLit” flag. If this flag is false, then the room is considered dark.~~
    4. ~~As a homage to Zork, if the player enters a dark room, the message “It is pitch black. You may are likely to be eaten by a grue”. If the player stays in the dark room, then the player will be eaten by said grue.~~
    5. ~~Add doors to the rooms that can be locked, requires the right key to go that direction.~~
    6. ~~Update driver to check door locks~~
    7. ~~Update Builder to be able to handle these additions~~
03. Simple Combat
    1. Add weapon items for combat
    2. Add Simple hostile NPCs
    3. Update the Builder to be able to build NPCs and weapon items
    4. Update the Driver to be able to handle simple combat where the player and NPC take turns trying to attack each other until one dies.
04. World Saves
    1. Get the Driver to be able to save the state of the world to a file separate from the original source file.
    2. Update the builder to take in a save file (should be a small update)
05. Simple Events
    1. Get the Event Handler to handle simple triggers: turn count, bringing item into a room, event trigger count, item used, player death.
    2. Get the Event Handler to handle simple actions: display a message, change a pathway of a room, move item, (un)lock a door.
    3. Update the Builder for users to add events.
06. Build Menu
    1. Add the Build Menu to the Builder that steps the user through making their world.
07. Beta Testing
    1. Have users test the program for any bugs and get feedback for possible changes.
08. Complex Worlds & Items
    1. Add stationary items that cannot be picked up, only used (like a light switch)
    2. Add container items (stationary and mobile)
    3. Add NW, NE, SW, and SE Directions
    4. Add ability to have doorways be hidden or visible.
    5. ~~Add hit-chance to weapons and hostile NPCs.~~
09. Complex Events
    1. Add ability to have logic statements with conditionals.
    2. Add additional triggers and actions (to be determined)
    3. Add persistence to events.
10. Evaluate Feedback
    1. Evaluate the user feedback and determine if/what should be added or changed.
    2. Make any necessary changes
