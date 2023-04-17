# Patch Notes: (04/xx/23)

## v0.3.x.x

### Changes

- Simple items the player can "take" and "drop"
  - ~~Simple Items consist of the following Item Types: weapons, light sources, keys~~
- The player can now use the actions "take" and "drop" for items
  - The player now has an inventory to hold items, with a limit of 7 items
- ~~Rooms have a “isLit” flag. If this flag is false, then the room is considered dark.~~
  - As a homage to Zork, if the player enters a dark room, the message “It is pitch black. You are likely to be eaten by a grue”. If the player stays in the dark room, then the player will be eaten by said grue.
  - ~~Doors have been added to the rooms that can be locked~~
  - ~~Doors have a presence-phrase, a name, a lock integer and a pointer to another room~~
    - T~~he presence-phrase is for when the driver announces the room. Instead of the possible directions being in the Room's description, that will be handled by the driver~~
      - ex. For NORTH door named "chained gate" /w a pres-phrase of "There is a", after the Room's description the Driver would output
      - \>"There is a chained gate to the NORTH"
    - To enter a locked "door", the door's lock number and the key's value must match. A value of 0 is considrered unlocked
    - Just like if a direction is set to NULL, if a door is locked the player will remain in the current room
- Updates to the Builder:
  - ~~A new section "#items" is now expected to be seen~~
    - There does not have to be any defined items, but the items header must always be present
  - ~~Each direction for a room is on its own line. These lines hold the information for its respective door~~
- ~~The Proto-SourceFormatOverview has been updated to reflect what the Builder now expects~~
  - ~~The Fort.trksrc example file has been updated as well~~

### Bug Fixes

- **No Bug Fixes for this Patch**
