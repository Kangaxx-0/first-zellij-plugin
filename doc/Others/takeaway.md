
## Understanding the Environment

When developing plugins for Zellij, it's important to recognize the unique environment you are working in. Unlike traditional terminal applications, Zellij plugins operate within a WebAssembly (WASM) runtime, which introduces specific constraints and considerations:

### Cursor Visibility and Interaction:
- By design, the cursor in Zellij plugins is hidden by default. This is a conscious design choice rather than a limitation of the environment.
- As a developer, you should be aware that standard methods to control or interact with the cursor (like those used in typical terminal applications) may not apply here. But, you can assume the cursor always begins with (0,0), and you have the renderable screen size from `render` function

### Limitations on Terminal Interactions:
- Standard system calls or APIs that work directly with the terminal, such as getting the cursor position, might not function as expected in the Zellij plugin environment.
- This is primarily because of the intermediary WASM runtime layer which separates the plugin from direct host terminal interactions.

### No Direct Terminal State Access:
- Plugins do not have direct access to the terminal state. This means functionalities like querying or setting the terminal state, which are often available in conventional terminal applications, are not directly accessible in Zellij plugins.
