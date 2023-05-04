# Patch Notes: (05/03/23)

## v0.3.4.2

### Changes

- **No Functional Changes for this Patch**

### Bug Fixes

- Fixed an issue with the RNG
- Fixed an issue with NPC announcement

### Additional Notes

- **No "Additional Notes" for this Patch**

***

# Patch Notes: (04/21/23)

## v0.3.4.0

### Changes

- Simple Combat
  - Enemy NPCs have been added in.
  - Functionality for weapons has been added in.

### Bug Fixes

- **No Functional Changes for this Patch**

### Additional Notes

I am aware of some bugs already with the combat. However, I am going to consider this phase of the project complete, then fix those bugs and any others discovered after this patch.

***

# Patch Notes: (04/17/23)

## v0.2.7.1

### Changes

- **No Functional Changes for this Patch**

### Bug Fixes

- Room floors are now being initialized correctly
- Room announcement has been fixed, multiple valid pathways now get announced properly
- Fixed a couple bugs where taking or dropping items was not working properly
- Fixed a bug where locked doors where not barring the player from moving through a door
- Fixed a bug where the parser was not getting the item name properly (just a case-sensitive issue)

***

# Patch Notes: (04/17/23)

## v0.2.7.0

### Changes

- Simple items the player can "take" and "drop"
  - Simple Items consist of the following Item Types: weapons, light sources, keys
- The player can now use the actions "take" and "drop" for items
  - The player now has an inventory to hold items, with a limit of 7 items
- Rooms have a “isLit” flag. If this flag is false, then the room is considered dark.
  - As a homage to Zork, if the player enters a dark room, the message “It is pitch black. You are likely to be eaten by a grue”. If the player stays in the dark room, then the player will be eaten by said grue.
  - Doors have been added to the rooms that can be locked
  - Doors have a presence-phrase, a name, a lock integer and a pointer to another room
    - The presence-phrase is for when the driver announces the room. Instead of the possible directions being in the Room's description, that will be handled by the driver
      - ex. For NORTH door named "chained gate" /w a pres-phrase of "There is a", after the Room's description the Driver would output
      - \>"There is a chained gate to the NORTH"
    - To enter a locked "door", the door's lock number and the key's value must match. A value of 0 is considrered unlocked
    - Just like if a direction is set to NULL, if a door is locked the player will remain in the current room
- Updates to the Builder:
  - A new section "#items" is now expected to be seen
    - There does not have to be any defined items, but the items header must always be present
  - Each direction for a room is on its own line. These lines hold the information for its respective door
- The Proto-SourceFormatOverview has been updated to reflect what the Builder now expects
  - The Fort.trksrc example file has been updated as well

### Bug Fixes

- **No Bug Fixes for this Patch**

***

# Patch Notes: (03/17/23)

## v0.1.2.2

#### Changes

- Room pathways are now defined with the rest of the room definition, instead of in a separate section
- The number of rooms no longer needs to be defined, only the rooms themselves
- Defining a pathway with an ID that does not have an associated room now provides a warning to the user

#### Bug Fixes
- (Deprecated)Defining less rooms than is declared no longer causes the program to crash