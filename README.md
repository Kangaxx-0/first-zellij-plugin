# A plugin sample for zellij plugin system developers

### How to use

This sample plugin provides a `my.kdl`, the only difference from the zellij default config is a keybinding `Ctrl t t` to render our plugin


```
     bind "t" {
           LaunchOrFocusPlugin "file:[absolute-path]/my-first-zellij-plugin.wasm" {
               floating true
               move_to_focused_tab true
           };
           SwitchToMode "Normal"
       }
```

Change the file path from your local, and run `zellij -c <path-to>\my.kdl`
