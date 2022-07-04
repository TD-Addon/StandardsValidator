# Standards Validator
Tamriel Data and the projects using it set various standards for themselves.
Modders are unfortunately fallible creatures, so it pays to validate files once in a while.

All standards should be documented in the [PT Wiki](https://wiki.project-tamriel.com/wiki/Modding_guidelines) or the [TR Handbook](https://www.tamriel-rebuilt.org/tr-handbook).

# Usage
This project requires [Node.js](https://nodejs.org/en/) to run and expects input files created using [tes3conv](https://github.com/Greatness7/tes3conv).

1. Convert a section file (or Tamriel Data) to JSON using tes3conv:<br />
`tes3conv file.esp file.json`
2. Run the validator:<br />
`node path/to/validator.js path/to/file.json`
3. Determine if any of the reported issues need to be fixed and fix them
