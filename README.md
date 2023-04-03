# Rusty Tork

## Current Version: v0.1.2.2

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

### Most Recent Patch Notes (as of 3/17/23)

#### Changes

- Room pathways are now defined with the rest of the room definition, instead of in a separate section
- The number of rooms no longer needs to be defined, only the rooms themselves
- Defining a pathway with an ID that does not have an associated room now provides a warning to the user

#### Bug Fixes

- (Deprecated)Defining less rooms than is declared no longer causes the program to crash
