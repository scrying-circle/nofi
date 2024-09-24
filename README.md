# nofi2

The second revision of Nofi.
This tool offers a fast and easy-to-use rendition and evaluation of Noita's spells.

## How to open
### Basics
#### Note: it is good practice to build applications from source(instructions below). However, we provide a pre-built release for Linux out of convenience.
To run this tool, download the latest release, and run the executable `nofi2`. You MUST provide the data-file path as an absolute command line argument, so that the tool can obtain the required assets. This can be done as follows:
```
./nofi2 --data-path /home/your/path/here/Nolla_Games_Noita/
```
The other arguments are optional, but a mod path is necessary to have access to evaluation features.

### Non-linux usage
Windows: Fully compatible using the below build steps.
MacOS: ?

## Example Usage
![image](https://github.com/user-attachments/assets/3cec23c2-f557-4a38-9380-0a5aaeb03f82)

## Configuration
Running this excecutable with `-h` provides a list of arguments this tool accepts. The first few relate to the rendering process, and all others assist with how the spells are evaluated. All paths should be relative to the executable. The mentioned mod- and data- file paths should point to the directories containing these Noita folders.
## Building the Application
Clone the repository, then run:
```
flutter build [linux|android|windows|chrome|idk]
```
The executable will be located at `build/..somethingsomething../release/bundle`. For example, on linux, the path is as follows: `build/linux/x64/release/bundle`. It is important to keep the executable in the `bundle` directory, as it must access external files, but the directory itself can be moved freely. There is also an executable at `bundle/data/flutter_assets/assets/circles_nofi_clip_tool`. Instructions to build this from source can be found [here](https://github.com/scrying-circle/fantastic-octo-enigma). (Windows) Note that this binary must be renamed to `circles_nofi_clip_tool_windows.exe` before placing it in the `assets` folder.
## Controls
`Ctrl+[i]` selects the `i`th autocomplete suggestion from the top. `Enter` copies the spell render to clipboard, and `Ctrl+Enter` copies the wand evaluation, formatted for Discord. `Esc` closes the program immediately. Appending `/` to a spell will add the zero-charges icon to it.
## Dependencies
Requires `luajit` to be installed.
## Spell Alias Dictionary
This tool accepts spell ID to determine which spells to render. Since these are often unintuitive, there is a handwritten default dictionary that aliases most spells to a short and/or intuitive form. This can be edited or the path can be changed to your own text file using the `-d` command line argument. I would recommend following exactly the same format as the provided one for any custom-written ones to avoid unexpected behaviour.
## Troubleshooting
If things are failing to be copied to clipboard, check that `bundle/data/flutter_assets/assets/circles_nofi_clip_tool` is set to be 'executable as a program'.
# Credits
NathanSnail for the [Lua script](
https://github.com/NathanSnail/wand_eval_tree.git) to evaluate the spells.
Fizzy for extensive assistence with testing Windows builds.
