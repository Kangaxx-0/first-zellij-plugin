## Why worker?
Every zellij plugin is compiled into WASM, and WASI doesn't provide native support for multi-threading or asynchronous I/O.

- Blocking the Main Thread: If you perform a long-running operation like scanning a disk within a function in your WASI module, it will block the main thread, potentially leading to poor system responsiveness.

- Lack of Asynchronous APIs: WASI has not provided asynchronous APIs or native thread support, so you can't easily offload the task to another thread.

- Resource Limitation: WebAssembly modules run in a constrained environment, which might have limited access to system resources.

## What does zellij do differently?

zellij worker is essentially a thread to take care a long-run job. In zellij server, it starts off a channel and move receiver to a new thread(this is a common sense in zellij code)

![diagram](../../assets/diagrams/Zellij_worker.png)


## Code Explained

The code primarily utilizes two structures - `FilesWorker` and `ModuleState` — to manage tasks asynchronously and update the plugin's state without blocking the main UI.

### Overview
- The `FilesWorker` structure is a Zellij worker responsible for performing file search operations in a separate thread. It sends a message back to the plugin once the task is completed.

- `ModuleState` is a Zellij plugin responsible for rendering the UI and handling various events. It manages the module's internal state, including the list of files (file_state).

### Workflow

Key Points:
- **Thread-Local**: Manages a thread-local variable (files) that tracks the files found in the given path.
- **Long-Running Job Simulation**: The function search simulates a long-running task that lasts for 10 seconds.

Functions:

- search: Searches for files in the given path and fills the files vector with filenames.
  1. Reads the directory of the given path and populates the files vector.
  2. Simulates a long-running task (10 seconds).
  3. Sends a "done" message to the plugin using `post_message_to_plugin`.

### Updating the module state

Instead of updating `file_state` directly, the following steps are taken:

1. `FilesWorker's search` function sends a "done" message using post_message_to_plugin. This message is wrapped into a CustomEvent by the host.
2. The plugin subscribes to `EventType::CustomMessage` and listens for the "done" message.
3. Upon receiving the "done" message in the update function, `file_state` is updated with the new list of files.

By separating the concerns of searching files and updating the UI, the plugin ensures that the main UI remains responsive while the long-running job is being processed.

--- 

# How to use

This sample plugin provides a `my.kdl`, the only difference from the zellij default config is a keybinding `Ctrl t t` to render our plugin


```
     bind "t" {
           LaunchOrFocusPlugin "file:[absolute-path]" {
               floating true
               move_to_focused_tab true
           };
           SwitchToMode "Normal"
       }
```

Change the file path from your local, and run `zellij -c <path-to>\my.kdl`
