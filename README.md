# Marcos

Command line file manager in Rust

Currently WIP to change terminal backend from tui-rs to cursive


## Design Goals

* Fast: The application should be snappier at startup times after reading all necessary configs, etc.
* Fully operational: _Most_ of the features of a morden file manager should be implemented.
* Keyboard Driven: The whole idea of the project is based upon the keyboard driven approach inspired from _vim_.
* Functional: Mounting and unmounting, disk usage analyzer, etc should be implemented to make a more usable console file manager.

## Possible future goals

* 24-bit Image: Preview image with the help of w3mimg.
* Async: Make the file read/write operations asynchronous.
* Extensibility: Marcos could be extended with the help of other plugins
* Multilingual plugins: Plugins can be written in various languages
* Support for popular cloud storage services.

## Architecture of marcos

Marcos contains various individual `tabs`, and will exit when the last tab closes. Each tab contains the following views: parent_view, current_view, preview. 
A status bar and bottom jumplist is common to all views. An editview is optinally shown when you input certain commands. To edit command simply press `:`
