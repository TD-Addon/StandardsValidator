# General advice
False positives are a possibility. Consult the [glossary](https://wiki.project-tamriel.com/wiki/Glossary) if you don't know what a word means.

# The primary validator (without `--extended`, `--names`, or `--fix-out-of-bounds`)

## Records

### Does not match a known ID scheme
This ID does not match the [guidelines](https://wiki.project-tamriel.com/wiki/Modding_guidelines#ID_Guidelines).

### Has a Tamriel Data ID
Either this is a dirty edit, or someone did not follow the guidelines.

### Is auto calculated
This spell is auto calculated which means that any auto calculated NPC can have it. Including vanilla NPCs.

### Is dead but does not have corpse persists checked
This actor is going to disappear after three days. Possibly without the player having ever seen the body.
This actor will also play a death scream when it enters the active grid.

### Is missing a sound gen
This creature does not have a sound gen defined.
This doesn't necessarily mean it doesn't play the right sound as the [rules](https://gitlab.com/OpenMW/openmw/-/issues/4813) are somewhat complicated,
but it might mean the creature uses the wrong sound effects.

### Shares its ID with a record of type X
This ID is used for multiple different things. Not generally harmful, but definitely confusing.

### Should have ID b_v_X_head_01
See [this bug](https://github.com/TD-Addon/TD_Addon/issues/110).

### References X
This object contains or creates an object that is supposed to be unique to the vanilla game.
This includes things like quest rewards and vendor chests.

### Is not calculated for all levels
This levelled list does not have `Calculate from all levels <= PC's level` checked, despite containing entries of different levels.
Any entries in this list set to spawn at levels below the entries with the highest level smaller than or equal to the player's level will not appear.
For example, if a list contains `1 Rat` and `2 Cliff Racer`, it will never spawn any rats when the player's level is higher than 1.
(It will still spawn cliff racers at levels higher than 2.)

### Which will not resolve to anything at that level
Levelled list resolution is recursive. If one list contains another, the conditions for both lists must be met before anything is spawned.

### Has a missing icon/mesh/name
This field should exist.

### Has invalid icon/mesh
The path is missing a `.` and is therefore probably not correct.

## References

### Persistent object used multiple times
[Persist objects](https://wiki.project-tamriel.com/wiki/Scripting#References_Persist) are meant to be used in scripts.
There should only ever be one instance of these objects. If an object is persistent but not used in any scripts, it should not be persistent.

### Duplicate references
Having two of the same object occupying the same position is redundant. The warning contains the position of both references in case the distance threshold is raised.

### Contains out of bounds reference
This exterior cell contains a reference that, by its position, should be part of another cell.

### Contains broken reference
There is something wrong with this object and it should not be used.

### Contains deprecated reference
This reference is deprecated and should not be used.

### Contains above water black square
Black squares are used to hide things on the minimap (as black blends in with the fog of war's color as well as the there-is-nothing-here-background's color.)
This doesn't work very well in cells with water as those get a water texture for a background instead of the usual black.
Which makes the black squares stand out like a sore thumb if placed above the water.

### Contains an unlinked PrisonMarker
This cell contains a `PrisonMarker` that doesn't link to an interior cell. `PrisonMarker`s need to link to interiors containing `stolen_goods` to prevent crashes.

## Supply chests

### Not available to all ranks
Supply chests should be available to all faction members.

### Not owned by the faction
Supply chests should use a unique record for each faction and they should always be owned by the faction they were made for.

## NPCs
See the [NPC guidelines](https://wiki.project-tamriel.com/wiki/Modding_guidelines#NPC_Guidelines).

### Does not have a script
Any NPC not on Vvardenfell must have a script. NPCs on Vvardenfell may still be assigned scripts.

### Uses unknown script
The script this NPC uses is not defined in this file and does not start with `T_`. It may or may not have the requisite variables.

### Uses script which does not define
The script this NPC uses is missing one or more required local variables.

### Defines T_Local_Khajiit but is not used by any khajiit
The script is only applied to non-khajiit NPCs and therefore does not require this local variable.

### Has class X, which should be Y
This NPC has a class that is not suitable for use in this province.

### Has auto calculated stats and spells
Spells have names that are visible to the player (either when buying them or when being affected by them.)
These names imply a certain culture, meaning NPCs outside of Morrowind should not receive vanilla spells.

### Is not using unique head/hair
This NPC should be using the asset designed for it.

### Is using head/hair
This head or hair is inappropriate for use by this NPC; usually for cultural reasons.

### Reports crimes despite having >= 70 fight
This NPC is likely to be hostile, but has an alarm of >= 100, meaning the player can get a bounty fighting them.

### Does not report crimes despite being a guard
This NPC has the guard class but does not quite function as a guard.

### Is not using animation epos_kha_upr_anim_X.nif
This NPC is not using upright Khajiit animations.

### Has animation epos_kha_upr_anim_X.nif
This NPC is using upright Khajiit animations despite not being one of the Khajiit races that requires them.

### Has multiple slave bracers
Slaves should generally only wear a single slave bracer.

### Knows spell
This NPC knows a spell that is culturally or geographically inappropriate.

### Is a vampire but uses head
This NPC is a vampire but does not use the correct vampire head for its race.
NPC vampires that do not need to switch to a mortal appearance should use their race's vampire head as their default head to ensure they always look like vampires.

## Keys
A misc item is a key if it has the key flag. This is a property of the record and determines if it can be sold to merchants and detected by Detect Key.

### Key not defined in this file
If the key record is not defined in this file, it might not have been flagged as a key by the CS.

### Is not a key
This misc item has `key` in its ID, but is not flagged as a key.

## Books

### Contains invalid HTML opening/closing tag
Morrowind only supports &lt;div&gt; &lt;font&gt; &lt;br&gt; &lt;p&gt; &lt;img&gt; &lt;b&gt; any other tags should just be removed.

### Contains invalid IMG SRC
Book art paths need to use `\` not `/` to work from BSAs in Morrowind.exe.

### Contains invisible text
Text not followed by a tag is not displayed in game. Every book should end in &lt;br&gt; to prevent this.

## Services

### Buys magic items but does not have a barter menu
Flagging an NPC or Creature as being a vendor of magic items does not enable the barter button in the dialogue window. This requires them to be a vendor of one of the other types.

### Does not barter
A class or NPC that implies barter services does not offer them.

### Does not have any barter gold
This NPC barters but does not have any gold to buy items with.

### Has barter gold but does not barter
This NPC might have been intended to barter, but doesn't.

### Does not offer travel services
Certain classes have voice lines and greetings that imply they can transport the player.
If an NPC with such a class does not offer travel services, they should be assigned another class.

### Does not have a reply to the destination topic
Travel NPCs should be able to tell you about where they can take you.

### Offers travel to X but there is no return travel there
This travel connection is a one-way trip.

### Offers X travel to Y but there is no corresponding return travel
One of these NPCs likely has the wrong class. A guild guide and shipmaster should not be on the same network even if they share a cell, for example.

### Does not mention in their destination response
This NPC has a response to the destination topic that doesn't mention every cell name they offer travel to.

## Orphaned objects
These are probably leftovers from previous releases. Of course, they might also be intended for an unfinished or unmerged quest.

### Script never started
A script that is never started is likely to be unused. It is possible to use a script as a collection of variables, akin to declaring global variables.
In this case the script is not unused, but the approach should still be reworked.

### Unused records
A record is considered unused if it is not present in the world, not in any inventory or leveled list, and not spawned via script.
This check is not performed in TD mode.

Be aware that, when checking a claim file, it is possible for the record to be used by the section file the claim is meant to be merged into.

## Text

### Contains odd character
These characters tend not to look very good in game.

### Contains a single hyphen
Vanilla sometimes uses one `-`, and sometimes it uses `--`. We always use the latter.

### Contains a short/an overlong ellipsis
An ellipsis should be composed of three dots.

### Contains double spaces
This is a game, not a piece of paper composed on a typewriter.

### Contains doubled up punctuation
A typo that should be fixed.

### Contains punctuation preceded by whitespace
Common in certain languages, but not in English.

### Contains an indefinite article followed by a PC variable
`%PCName`, `%PCRace`, `%PCRank`, and `%PCClass` can be anything (although the latter two require 3rd party mods to be wholly unknowable)
which means it's impossible to determine which indefinite article (a/an) to use. It is possible to do the latter two correctly (mods aside) but mistakes are common.

## Dialogue

### Has an unnecessary (Not) class/faction/race/sex/id filter
This line that is assigned to a specific actor has a filter that checks something about that actor that cannot ever change.

### Has a NoLore/T_Local_NoLore/T_Local_Khajiit/T_Local_NPC filter
Like the above, but for local variables.

### Does not have a T_Local_NoLore filter
This line is available to NPCs who should not have lore.

### Has a Not Local NoLore filter
All NPCs added by these projects should have a NoLore variable meaning this line is not available to them.

### Has a Local/Not Local X filter
This variable comparison is incorrect.

### Has a class filter
This class is not appropriate for use in this province. The line should be inaccessible.

### Does not have a known project specific local filter
This line is missing a filter that unambiguously ties it to the project, meaning it might show up on NPCs added by other mods.
Filter the line to a local variable that starts with your province's ID prefix or use one of the T_D variables.

If this line is meant to override a vanilla line that does not have a NoLore filter, add `;SV: overrides vanilla` to the result script.

### Checks for Dead = X
This line only fires if the death count is exactly X. Which means it can break if players use `resurrect` or the NPC gets cloned.

### Has no text
So why does it exist?

If this line was intentionally left blank for result scripting purposes, add `;SV: intentionally left blank` to the result script.

## Scripts

### Contains line short/long/float X
This line declares a local variable that shares its name with a built-in function, most commonly `OnActivate`.
This can break other scripts attempting to use that function if they were compiled later.

### Uses Position instead of PositionCell
This script uses the `Position` function which can cause issues in Morrowind.exe. `PositionCell` should be used instead.

### Contains non-standard khajiit check
NPC scripts can be divided into three categories: scripts that are only applied to Khajiit, scripts that are never applied to Khajiit,
and scripts that are sometimes applied to Khajiit. Scripts that are only ever applied to Khajiit should set `T_Local_Khajiit` to 1
without doing a race check. Scripts that are never applied to Khajiit should not contain the `T_Local_Khajiit` variable at all. And
scripts that are applied to both Khajiit and non-Khajiit should contain the standardized script snipped that checks for each Khajiit race.

### Sets T_Local_Khajiit multiple times
This script doesn't implement the standardized Khajiit script, but does set the variable multiple times.

### Contains unexpected line set T_Local_Khajiit to X
This script sets the variable to an unexpected value.

## Magic

### Uses effect
This magic effect should not be used. Either because it's Corprus related or for reasons of mod compatibility.

### Uses effect without a magnitude
This magic effect needs a magnitude to be useful. Might be fine if the spell is used for its visual effect.

### Uses effect with duration 1
This magic effect needs a duration to be useful. Might be fine if the spell is only meant to be detected by a script.

## Cells

### Has a fog density of 0
This can cause [graphical issues](https://en.uesp.net/wiki/Morrowind_Mod:Fogbug).

### PathGrid contains underwater node
Path grids that are underwater aren't used.

### PathGrid contains duplicate node
There's two nodes in the same spot. Delete one and fix the connections.

### PathGrid contains unconnected node
Nodes that aren't connected to any other nodes are useless.
If you have a single-tile room with a locked door, just omit the path grid instead of adding a single unconnected node.

# The extended validator (`--extended`)

## Ownership checks
Items in dungeons should not have owners. Items in towns should.
There is no easy way to determine what's a dungeon and what's a town, so this check is liable to produce a number of false positives.
Such as a chest owned by a friendly NPC standing by the side of the road. Or a container plant in a public square.
It will also report cases where ownership has been applied overzealously.
Some of these, like unnamed activators, don't matter or only matter in unlikely circumstances.
It would be best to fix these anyway, if only to reduce file size.

## Cell does not contain any NPCs or creatures
It is legal to sleep in this cell, yet it does not contain any enemies. This suggests an unpopulated dungeon.

## Cell is missing a path grid
This interior cell does not have a path grid.
Given that players can Command or Summon other actors into any cell, this is only correct if the entire cell is underwater or especially small.

## Scale check
Items the player can pick up should not be resized in the CS as picking them up resets their size.

## Deprecated objects
The extended validator scans cells, inventories, scripts, dialogue filters, and leveled lists for deprecated objects, factions, and classes.
Taking into account objects marked as deprecated in the CSSE toml file, as well as objects set up to use the deprecated cube, and objects with "deprecated" in their name.