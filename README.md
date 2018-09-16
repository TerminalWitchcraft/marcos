# Marcos [![Build Status](https://travis-ci.org/TerminalWitchcraft/marcos.svg?branch=master)](https://travis-ci.org/TerminalWitchcraft/marcos)

Command line file manager in Rust with VIM-inspired keybindings.

## Key bindings

Only some key bindings are implemented as of now. All mentioned key bindings will be implemented before `0.1.0` release.

| Key      | Action                                                                                |
|----------|---------------------------------------------------------------------------------------|
| q        | Exit marcos                                                                           |
| j        | Select item down                                                                      |
| k        | Select item up                                                                        |
| h        | Go previous (left)                                                                    |
| l        | Go next(right)                                                                        |
| :        | Activate command mode                                                                 |
| gg       | Go to the first selection                                                             |
| G        | Go to the last selection                                                              |
| [count]G | Go to the [count] item                                                                |
| za       | Toggle visibility of hidden items                                                     |
| y        | Yank(Copy) the selected file/folder(Similar to Ctrl-c)                                |
| x        | Cut the selected file/folder(similar to Ctrl-x)                                       |
| p        | Paste the Copied/Cut file/folder(Similar to Ctrl-v)                                   |
| r        | Rename selected file/folder                                                           |
| dd*      | Delete selected file/folder(with confirmation)                                        |
| o        | Create new file(`touch filename`)                                                     |
| O        | Create new directory (`mkdir dirname`)                                                |
| P        | Paste the Copied/Cut file/folder replacing existing with same name(with Confirmation) |
| mX       | Create a bookmark with name X                                                         |
| `X       | Jump to bookmark with name X                                                          |
| n        | Move to next match                                                                    |
| N        | Move to previous match                                                                |
| /        | Search                                                                                |
| v        | Starts visual mode, selects all files until you press ESC                             |
| V        | Visual mode, select all                                                               |
| Ctrl+r   | Refresh(listings, data, cache, etc)                                                   |
| ESC      | Get me out!                                                                           |


## Design Goals

* Fast: The application should be snappier at startup times after reading all necessary configs, loading resources, etc.
* Fully operational: _Most_ of the features of a morden file manager should be implemented, although support for uncommon features should be offloaded to scripts/plugins
* Keyboard Driven: The whole idea of the project is based upon the keyboard driven approach inspired from _vim_.
* Functional: Composition/chaining of vim-like commands based on your existing muscle memory, to just get things done. I am not inclined towards defining new set of keybindings/commands until unless it is absolutely necessary!

## Possible future goals

* 24-bit Image: Preview image with the help of w3mimg(or possible [ueberzug](https://github.com/seebye/ueberzug)).
* Async: Make some of the core operations non-blocking
* Extensibility: Marcos could be extended with the help of plugins
* Support for lua/python plugins: Plugins can be written in lua/python languages

## Architecture

The best way to learn and know about the structure is by reading the docs by running:
```
cargo doc --open
```

Besides, I try my best to include relevant comments and TODO[detail] tags.
