# Standards Validator
Tamriel Data and the projects using it set various standards for themselves.
Modders are unfortunately fallible creatures, so it pays to validate files once in a while.

All standards should be documented in the [PT Wiki](https://wiki.project-tamriel.com/wiki/Modding_guidelines) or the [TR Handbook](https://www.tamriel-rebuilt.org/tr-handbook).

## Usage
This project requires [Node.js](https://nodejs.org/en/) to run and expects input files created using [tes3conv](https://github.com/Greatness7/tes3conv).

1. Convert a section file (or Tamriel Data) to JSON using tes3conv:<br />
`tes3conv file.esp file.json`
2. Run the validator:<br />
`node path/to/validator.js path/to/file.json [mode]`<br />
Where [mode] is optionally one of PT/TR/TD depending on which specific validation rules you need.
3. Determine if any of the reported issues need to be fixed and fix them

## Ownership checking
Items in dungeons should not have owners. Items in towns should.
There is no easy way to determine what's a dungeon and what's a town, so this check is liable to produce a number of false positives.
Such as a chest owned by a friendly NPC standing by the side of the road. Or a container plant in a public square.
It will also report cases where ownership has been applied overzealously.
Some of these, like unnamed activators, don't matter or only matter in unlikely circumstances.
It would be best to fix these anyway, of only to reduce file size.

### Usage
Because the check requires knowledge of object types, it needs more data to work with than just the file that needs checking.
As such, every dependency of the checked file must be passed as an argument. The last file in the list will be checked.

`node ownership.js Morrowind.json Tribunal.json Bloodmoon.json Tamriel_Data.json TD_Addon.json file.json`
