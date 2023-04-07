# Patch Notes: (04/xx/23)

## v0.2.4.0

### Changes

- Rooms' pathways array of Room pointers have been replaced with an array of doorways, which contain a lock integer, a name, an article-phrase, and a pointer to a room.
  - Proto-SourceFormatOverview.txt file has been updated
- Rooms have a "dark" bool that flags if a room is lit or not
- Simple Items added: keys, weapons, light sources
  - the Builder has been updated to parse the information for items
  - ~~player can now use "take" and "drop" for adding or removing items to inventory~~
  - ~~player now has an inventory, the current limit is 7~~

### Bug Fixes

- **No Bug Fixes for this Patch**
