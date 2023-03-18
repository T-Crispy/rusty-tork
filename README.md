# Rusty Tork

## Current Version: v0.1.2.2

### Project Description

This project for an Interactive Fiction game engine called “Tork”. 
Interactive Fictions (IFs) are text based games that allow a player to type an input in order to interact with the world. 
One IF that gained quite a bit of popularity at one point is the Zork series. 
The aim of Tork will be to allow users to create their own “Zork” game.

### Most Recent Patch Notes (as of 3/17/23)

#### Changes

- Room pathways are now defined with the rest of the room definition, instead of in a separate section
- The number of rooms no longer needs to be defined, only the rooms themselves
- Defining a pathway with an ID that does not have an associated room now provides a warning to the user

#### Bug Fixes
- (Deprecated)Defining less rooms than is declared no longer causes the program to crash
