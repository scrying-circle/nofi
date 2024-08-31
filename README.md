# nofi2

The second revision of Nofi.
This tool offers a fast and easy-to-use rendition and evaluation of Noita's spells.

## How to open
### Basics
To run this tool, go to `build/linux/x64/release` and download `bundle`(you might find [this](https://download-directory.github.io/) useful). Run the executable `nofi2`:
```
./nofi2
```
### Configuration
Running this excecutable with `-h` provides a list of arguments this tool accepts. The first few relate to the rendering process, and all others assist with how the spells are evaluated. All paths should be relative to the executable.
### Non-Linux Operation
You're gonna have to install Linux. Just kidding, you can clone the repository and run `flutter build [your operating system here]` if it's supported, and then the binary & assets will be somewhere.

## Controls
`Ctrl+[i]` selects the `i`th autocomplete suggestion from the top. `Enter` copies the spell render to clipboard, and `Ctrl+Enter` copies the wand evaluation, formatted for Discord. `Esc` closes the program immediately.
## Spell Alias Dictionary
This tool accepts spell ID to determine which spells to render. Since these are often unintuitive, there is a handwritten default dictionary that aliases most spells to a short and/or intuitive form. This can be edited or the path can be changed to your own text file using the `-d` command line argument. I would recommend following exactly the same format as the provided one for any custom-written ones to avoid unexpected behaviour.
## Troubleshooting
If things are failing to be copied to clipboard, check that `bundle/data/flutter_assets/assets/clip` is set to be 'executable as a program'.
# Credits
NathanSnail for the [Lua script](
https://github.com/NathanSnail/wand_eval_tree.git) to evaluate the spells.
