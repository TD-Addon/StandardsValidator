# Standards Validator
Tamriel Data and the projects using it set various standards for themselves.
Modders are unfortunately fallible creatures, so it pays to validate files once in a while.

All standards should be documented in the [PT Wiki](https://wiki.project-tamriel.com/wiki/Modding_guidelines).

## Usage
The validator is a command line tool. Run with `--help` for up to date information.

1. Open your command line of choice (i.e. CMD, PowerShell, Bash, etc.)
2. Run the validator:<br />
`StandardsValidator.exe [mode] path/to/file.esm`<br />
Where [mode] is one of PT/TD/TR/Vanilla depending on which specific validation rules you need.
3. Determine if any of the reported issues need to be fixed and fix them

# Extended Validator
Some issues require more context to detect. More specifically, they require knowledge of your plugin's master files.
As such, every dependency of the checked file must be passed as an argument. The last file in the list will be checked.

`StandardsValidator.exe --extended [mode] Morrowind.esm Tribunal.esm Bloodmoon.esm Tamriel_Data.esm file.esp`

Note that by default the validator will attempt to load `file.esp`'s masters from the same directory meaning the above can be shortened to:

`StandardsValidator.exe --extended [mode] file.esp`

The `--disable-master-loading` flag can be used to disable this behaviour. The [mode] argument does nothing in this mode at this time.

# Warnings
An explanation of the various messages reported by the validators can be found in [WARNINGS](./WARNINGS.md).

# Out of bounds fixer
To automatically send cell references to the correct exterior cell:

`StandardsValidator.exe [mode] inputfile.esp --fix-out-of-bounds outputfile.esp`

# Name similarity
This check computes the Levenshtein distance between NPC names. It also checks if quest names are reused across different files.

`StandardsValidator.exe --names [mode] Morrowind.esm Tribunal.esm Bloodmoon.esm Tamriel_Data.esm file.esp`

Like `--extended` above, this mode attempts to load master files automatically.
