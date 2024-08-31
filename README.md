# nofi2

The second revision of Nofi.

## How to open
### Basics
To run this tool, go to `build/linux/x64/release` and download `bundle`. Run the executable `nofi2`:
```
./nofi2
```
### Configuration
Running this excecutable with `-h` provides a list of arguments this tool accepts. The first few relate to the rendering process, and all others assist with how the spells are evaluated. All paths should be relative to the executable.
### Non-Linux Operation
You're gonna have to install Linux. Just kidding, you can clone the repository and run `flutter build [your operating system here]` if it's supported, and then the binary & assets will be somewhere.

## Controls
`Ctrl+[i]` selects the `i`th autocomplete suggestion from the top. `Enter` copies the spell render to clipboard, and `Ctrl+Enter` copies the wand evaluation, formatted for Discord. `Esc` closes the program immediately.

## Troubleshooting
If things are failing to be copied to clipboard, check that `bundle/data/flutter_assets/assets/clip` is set to be 'executable as a program'.
# Credits
NathanSnail for the [Lua script](
https://github.com/NathanSnail/wand_eval_tree.git) to evaluate the spells.
