# Marcos

Command line file manager in Rust with VIM-inspired keybindings.



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
