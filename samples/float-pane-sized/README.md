## Prerequisites:
This plugin has a dependency of [zellij clone](https://github.com/Kangaxx-0/zellij), it relies on new API `resize_floating_pane_by_percent`

## How it works:
This plugin is able to resize any floating pane by given percentage


## What can you learn from this sample
This plugin is designed and implemented because the built-in floating pane does not support the functionality I want. As the section of Prerequisites says this plugins depends on the new interface, lots of required knowledge can be found in [wasm](../../doc/wasm.md) and [protobuf](../../doc/protobuf.md) sections

From UI perspective, When you design your plugin, you always need to keep in mind:

- A "screen" refers to a virtual workspace that can contain multiple tiles. Each screen in Zellij is like a separate desktop environment that can be customized with its own layout and collection of tiles.
- Tabs in Zellij allow you to switch between these different terminal sessions within each tile. For example, if you have two different shell sessions running in a single tile, you can use the tabs feature to switch between them. This allows you to manage multiple terminal sessions more efficiently and reduces the need to switch between separate terminal windows.
- a "tiled_pane" refers to a single pane or tile in the tiling window manager. A tiled_pane is a rectangular area of the terminal window that contains a single terminal session or command.
- A "float pane" is a special type of pane in Zellij that can overlap other panes.

 
