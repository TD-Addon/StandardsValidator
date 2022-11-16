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

# Extended Validator
Some issues require more context to detect. More specifically, they require knowledge of your plugin's master files.
As such, every dependency of the checked file must be passed as an argument. The last file in the list will be checked.

`node extendedvalidator.js Morrowind.json Tribunal.json Bloodmoon.json Tamriel_Data.json TD_Addon.json file.json`

# Warnings
An explanation of the various messages reported by the validators can be found in [WARNINGS](./WARNINGS.md).

# Out of bounds fixer
To automatically send cell references to the correct exterior cell:

`node oobfixer.js inputfile.json outputfile.json`

# Name similarity
This check computes the Levenshtein distance between NPC names.
It requires a dependency so `npm ci` must be used to install it before use.

`node names.js Morrowind.json Tribunal.json Bloodmoon.json Tamriel_Data.json TD_Addon.json file.json`
