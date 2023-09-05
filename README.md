# Zellij Plugin System: A Comprehensive Walkthrough 🛠️

Elevate Your Terminal Experience with Customizable Plugins!

Welcome to the definitive guide to the Zellij Plugin System! Have you ever wanted to extend your zellij functionalities but felt lost? We've got you covered! This repository serves as a hand-to-hand walkthrough that makes creating plugins for Zellij as easy as pie. 🥧

## 🌟 Features
- Step-by-Step Tutorials: Easy-to-follow guides that take you from zero to hero.
- Code Samples: Real-world examples to learn from and experiment with.
- Deep Dives: Get into the nitty-gritty of Zellij's plugin system.
- Best Practices: Learn the do's and don'ts of plugin development in Zellij.

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
