# WASI
zellij is using [wamer](https://wasmer.io/) as running for its plugin development. Zellij server plays the role of `Host`, and plugins are compiled to WebAssembly bytecode

### **Imports**

Imports in Wasmer are functions, memories, globals, or tables that are defined outside the WebAssembly module but can be used within the module. When you instantiate a Wasm module in Wasmer, you need to provide an "import object" that maps the names of these imported items to their actual implementations in the host environment.

For example, you might define an import object in Rust like this:

```
use wasmer::{imports, Function, Instance, Module, Store};

let import_object = imports! {
    "env" => {
        "my_function" => Function::new_native(&store, my_function),
    },
};
```

Here, **`"my_function"`** is a function that's defined in the host environment (i.e., in Rust) and is made available to the Wasm module.

### **Exports**

Exports are the opposite of imports: they are functions, memories, globals, or tables that are defined within a WebAssembly module and can be accessed from the host environment.

Once you've instantiated a Wasm module in Wasmer, you can access its exports like this:

```
let instance = Instance::new(&module, &import_object)?;
let my_exported_function: NativeFunc<(), ()> = instance.exports.get_native_function("my_exported_function")?;
my_exported_function.call()?;
```

Here, **`"my_exported_function"`** is a function that's defined within the Wasm module, and you can call it from the host environment.


### How does zellij and plugin transfer the data
WASI has provided virtual file system for its `stdin`, `stdout` and `stderr`, e,g - [wasi_write_object](https://github.com/zellij-org/zellij/blob/main/zellij-server/src/plugins/zellij_exports.rs#L1069) writes the data to STDIN, and [wasi_read_object](https://github.com/zellij-org/zellij/blob/main/zellij-server/src/plugins/zellij_exports.rs#L1076) reads data from its STDOUT;
for plugin module, there are a few functions can be used:
    - [object_from_stdin](https://github.com/zellij-org/zellij/blob/main/zellij-tile/src/shim.rs#L624)
    - [bytes_from_stdin](https://github.com/zellij-org/zellij/blob/main/zellij-tile/src/shim.rs#L633gg
    - [object_to_stdout](https://github.com/zellij-org/zellij/blob/main/zellij-tile/src/shim.rs#L641)

