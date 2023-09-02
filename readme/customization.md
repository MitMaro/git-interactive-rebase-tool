# Customization

## Table of Contents

* [Usage](./customization.md#usage)
* [Git Configuration](./customization.md#git-configuration)
* [General Settings](./customization.md#general)
* [Colors](./customization.md#colors)
* [Key Bindings](./customization.md#key-bindings)
* [External Editor](./customization.md#external-editor)

## Usage

The tool can be customized using the [git config](https://git-scm.com/docs/git-config) command. String values are case-insensitive.

### Example

    git config --global interactive-rebase-tool.foregroundColor black

## Git Configuration

Some values from your Git Config are directly used by this application.

| Key                                           | Description                                                             |
|-----------------------------------------------|-------------------------------------------------------------------------|
| [`core.commentChar`][coreCommentChar]         | Used when reading the TODO file to excluded commented lines             |
| [`core.editor`][coreEditor]                   | Used when deciding what editor to open when trigger the external editor |
| [`diff.context`][diffContext]                 | Used by show commit when generating a diff                              |
| [`diff.interHunkContext`][diffInterhunkLines] | Used by show commit when generating a diff                              |
| [`diff.renameLimit`][diffRenameLimit]         | Used by show commit when generating a diff                              |
| [`diff.renames`][diffRenames]                 | Used by show commit when generating a diff                              |

[coreCommentChar]:https://git-scm.com/docs/git-config#Documentation/git-config.txt-corecommentChar
[coreEditor]:https://git-scm.com/docs/git-config#Documentation/git-config.txt-coreeditor
[diffContext]:https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffcontext
[diffInterhunkLines]:https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffinterHunkContext
[diffRenameLimit]:https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffrenameLimit
[diffRenames]:https://git-scm.com/docs/diff-config/#Documentation/diff-config.txt-diffrenames

## General

| Key                           | Default | Type    | Description                                                                                 |
|-------------------------------|---------|---------|---------------------------------------------------------------------------------------------|
| `autoSelectNext`              | false   | bool    | If true, auto select the next line after action modification                                |
| `diffIgnoreBlankLines`        | none    | String¹ | If to ignore blank lines during diff.                                                       |
| `diffIgnoreWhitespace`        | none    | String¹ | If and how to ignore whitespace during diff.                                                |
| `diffShowWhitespace`          | both    | String² | If and how to show whitespace during diff.                                                  |
| `diffSpaceSymbol`             | ·       | String  | The visible symbol for the space character. Only used when `diffShowWhitespace` is enabled. |
| `diffTabSymbol`               | →       | String  | The visible symbol for the tab character. Only used when `diffShowWhitespace` is enabled.   |
| `diffTabWidth`                | 4       | Integer | The width of the tab character                                                              |
| `undoLimit`                   | 5000    | Integer | Number of undo operations to store.                                                         |
| `postModifiedLineExecCommand` |         | String  | Exec command to attach to modified lines. See [modified line exec command] for details.     |
| `verticalSpacingCharacter`    | ~       | String  | Vertical spacing character. Can be set to an empty string.                                  |

¹ Ignore whitespace can be:
- `change` to ignore changed whitespace in diffs, same as the [`--ignore-space-change`][diffIgnoreSpaceChange] flag
- `true`, `on` or `all` to ignore all whitespace in diffs, same as the [`--ignore-all-space`][diffIgnoreAllSpace] flag
- `false`, `off`, `none` to not ignore whitespace in diffs

² Show whitespace can be:
- `leading` to show leading whitespace only
- `trailing` to show trailing whitespace only
- `true`, `on` or `both` to show both leading and trailing whitespace
- `false`, `off`, `none` to show no whitespace

[modified line exec command]:../README.md#modified-line-exec-command
[diffIgnoreSpaceChange]:https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---ignore-space-change
[diffIgnoreAllSpace]:https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---ignore-all-space

## Colors

The valid colors are the [eight original 8 ANSI colors][ANSIColors]. They are `black`, `blue`, `cyan`, `green`, `magenta`, `red`, `white` and `yellow`. Dimmed versions of the 8 ANSI colors can be used by prefixing the color  with `dark`, for example `dark red`. Each terminal controls the exact color for these color names. On terminals that support 256 colors, a color triplet with the format `<red>,<green>,<blue>` can be used. Each color has a range of 0 to 255 with `255, 255, 255` resulting in white and `0,0,0` resulting in black. A value of `-1` or `transparent` can be used to use the default terminal color.

[ANSIColors]:https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit

| Key                       | Default      | Type  | Description                                              |
|---------------------------|--------------|-------|----------------------------------------------------------|
| `backgroundColor`         | Default¹     | Color | Color used for the background                            |
| `breakColor`              | white        | Color | Color used for the break action                          |
| `diffAddColor`            | green        | Color | Color used for lines and files added in a diff           |
| `diffChangeColor`         | yellow       | Color | Color used for lines and files changed in a diff         |
| `diffContextColor`        | white        | Color | Color used for context in a diff                         |
| `diffRemoveColor`         | red          | Color | Color used for lines and files removed in a diff         |
| `diffWhitespace`          | black        | Color | Color used for whitespace in a diff                      |
| `dropColor`               | red          | Color | Color used for the drop action                           |
| `editColor`               | blue         | Color | Color used for the edit action                           |
| `execColor`               | blue         | Color | Color used for the exec action                           |
| `fixupColor`              | magenta      | Color | Color used for the fixup action                          |
| `foregroundColor`         | Default¹     | Color | Color used for most text and the UI                      |
| `indicatorColor`          | cyan         | Color | Color used for text the indicates or needs to standout   |
| `labelColor`              | dark yellow  | Color | Color used for the label action                          |
| `mergeColor`              | dark yellow  | Color | Color used for the merge action                          |
| `pickColor`               | green        | Color | Color used for the pick action                           |
| `resetColor`              | dark yellow  | Color | Color used for the reset action                          |
| `rewordColor`             | yellow       | Color | Color used for the reword action                         |
| `selectedBackgroundColor` | 35,35,40     | Color | Color used as the background color for the selected line |
| `squashColor`             | cyan         | Color | Color used for the squash action                         |
| `updateRefColor`          | dark magenta | Color | Color used for the update-ref action                     |

¹ By default the foreground and background colors use the terminal defined colors

## Key Bindings

Most keys can be changed to any printable character or supported special character. It is possible to provide conflicting bindings, which will result in undefined behaviour. The `inputConfirmYes` binding has a special behaviour in that it responds to both the uppercase and lowercase letter of the value set, if the variant exist.

| Key                         | Default   | Type   | Description                                         |
|-----------------------------|-----------|--------|-----------------------------------------------------|
| `inputAbort`                | q         | String | Key for abort rebase with prompt                    |
| `inputActionBreak`          | b         | String | Key for setting action to rebase                    |
| `inputActionDrop`           | d         | String | Key for setting action to drop                      |
| `inputActionEdit`           | e         | String | Key for setting action to edit                      |
| `inputActionFixup`          | f         | String | Key for setting action to fixup                     |
| `inputActionPick`           | p         | String | Key for setting action to pick                      |
| `inputActionReword`         | r         | String | Key for setting action to reword                    |
| `inputActionSquash`         | s         | String | Key for setting action to squash                    |
| `inputActionIndex`          | i         | String | Key for setting action to index (git-revise)        |
| `inputConfirmNo`            | n         | String | Key for rejecting a confirmation                    |
| `inputConfirmYes`           | y         | String | Key for confirming a confirmation                   |
| `inputEdit`                 | E         | String | Key for entering edit mode                          |
| `inputForceAbort`           | Q         | String | Key for forcing an abort of the rebase              |
| `inputForceRebase`          | W         | String | Key for forcing a rebase                            |
| `inputHelp`                 | ?         | String | Key for showing the help                            |
| `inputInsertLine`           | I         | String | Key for inserting a new line                        |
| `inputMoveDown`             | Down      | String | Key for moving the cursor down                      |
| `inputMoveEnd`              | End       | String | Key for moving the cursor to the end of the list    |
| `inputMoveHome`             | Home      | String | Key for moving the cursor to the top of the list    |
| `inputMoveLeft`             | Left      | String | Key for moving the cursor left                      |
| `inputMoveRight`            | Right     | String | Key for moving the cursor right                     |
| `inputMoveSelectionDown`    | j         | String | Key for moving the selected line(s) down            |
| `inputMoveSelectionUp`      | k         | String | Key for moving the selected line(s) up              |
| `inputMoveStepDown`         | PageDown  | String | Key for moving the cursor down by a large step      |
| `inputMoveStepUp`           | PageUp    | String | Key for moving the cursor up  by a large step       |
| `inputMoveUp`               | Up        | String | Key for moving the cursor up                        |
| `inputOpenInExternalEditor` | !         | String | Key for opening the external editor                 |
| `inputRebase`               | w         | String | Key for rebasing with confirmation                  |
| `inputRedo`                 | Control+y | String | Key for redoing the previous undone change          |
| `inputRemoveLine`           | Delete    | String | Key for removing selected commits                   |
| `inputScrollDown`           | Down      | String | Key for scrolling the view down                     |
| `inputScrollEnd`            | End       | String | Key for scrolling the view to the bottom            |
| `inputScrollHome`           | Home      | String | Key for scrolling the view to the top               |
| `inputScrollLeft`           | Left      | String | Key for scrolling the view to the left              |
| `inputScrollRight`          | Right     | String | Key for scrolling the view to the right             |
| `inputScrollStepDown`       | PageDown  | String | Key for scrolling the view one step down            |
| `inputScrollStepUp`         | PageUp    | String | Key for scrolling the view one step up              |
| `inputScrollUp`             | Up        | String | Key for scrolling the view to the up                |
| `inputShowCommit`           | c         | String | Key for showing the overview of the selected commit |
| `inputShowDiff`             | d         | String | Key for showing the diff of the selected commit     |
| `inputToggleVisualMode`     | v         | String | Key for toggling visual mode                        |
| `inputUndo`                 | Control+z | String | Key for undoing the previous change                 |

### Example

    # single value
    git config --global interactive-rebase-tool.inputRebase S
    # multiple values
    git config --global interactive-rebase-tool.inputRebase "S L K"

### Supported Special Keys

Keys that do not have easily printable characters, such as the arrow keys, are set using the special values defined in the table below. Some special keys do not work correctly on some setups.

| Key         | Description     |
|-------------|-----------------|
| `BackTab`   | Back tab key    |
| `Backspace` | Backspace key   |
| `Delete`    | Delete key      |
| `Down`      | Down arrow key  |
| `End`       | End key         |
| `Enter`     | Enter key       |
| `Esc`       | Escape key      |
| `F{1..255}` | Function X key  |
| `Home`      | Home key        |
| `Insert`    | Insert key      |
| `Left`      | Left arrow key  |
| `PageDown`  | Page down key   |
| `PageUp`    | Page up key     |
| `Right`     | Right arrow key |
| `Tab`       | Tab key         |
| `Up`        | Up arrow key    |

### Modifier Keys

Most bindings can be prefixed with `Shift`, `Control` or `Alt`, followed by a `+`. These bindings can also be combined in any order, for example `Alt+Control+Delete`. `Shift` combined with any ASCII alphabetic character will not work and the uppercase character must be used instead.

### Multiple Key Bindings

A single action can have multiple bindings by providing a whitespace separate list of keys. For example the binding, `"u Control+z Control+Z"` would respond to a keypress of `a`, `Control z` and `Control Z`.

### Restricted Keys

Some key combinations are restricted as they have special meaning. They are:

| Key         | Description                                                                 |
|-------------|-----------------------------------------------------------------------------|
| `Control+c` | Immediately aborts the program without writing the rebase todo file to disk |


## External Editor

The external editor action will first attempt to start the editor defined by the [Git configuration "core.editor"][git-core-editor], followed by the `VISUAL` and `EDITOR` environment variables. Finally, if neither is set, the external editor defaults to using `vi`.

The `%` character in the value will be replaced with the rebase todo file. If the `%` character is not found, then the git rebase todo file will be provided as the last argument.

[git-core-editor]:https://www.git-scm.com/book/en/v2/Customizing-Git-Git-Configuration#_core_editor
