# WASI
zellij is using [wasmer](https://wasmer.io/) as running for its plugin development. Zellij server plays the role of `Host`, and plugins are compiled to WebAssembly bytecode

### **Imports**
Imports in WASI are functions, memories, globals, or tables that are defined outside the WebAssembly module but can be used within the module. When you instantiate a Wasm module in Wasmer, you need to provide an "import object" that maps the names of these imported items to their actual implementations in the host environment.

In Zellij, this mapping of host environment features to WebAssembly modules is facilitated through the [shim.rs](https://github.com/zellij-org/zellij/blob/main/zellij-tile/src/shim.rs) file, a lot of interfaces calls the below function through FFI.

```
#[link(wasm_import_module = "zellij")]
extern "C" {
    fn host_run_plugin_command();
}
```
Within shim.rs, a number of host functions are exposed and can be called from WebAssembly modules via the Foreign Function Interface (FFI).

Under the hood, below macro is defined in zellij code base.
```
pub fn zellij_exports(
    store: &Store,
    plugin_env: &PluginEnv,
    subscriptions: &Arc<Mutex<Subscriptions>>,
) -> ImportObject {
    imports! {
        "zellij" => {
          "host_run_plugin_command" => {
            Function::new_native_with_env(store, ForeignFunctionEnv::new(plugin_env, subscriptions), host_run_plugin_command)
          }
        }
    }
}
```

### **Exports**
Exports are the opposite of imports: they are functions, memories, globals, or tables that are defined within a WebAssembly module and can be accessed from the host environment.
Once you've instantiated a Wasm module in Wasmer, you can access its export function. 

In zellij, modules are exposing 4 functions that host can interact with:

- `load`: An entry point for initializing the plugin, this exported function gets called when desired plugin is loading from host.

- `update`: Be invoked when there's an event that the plugin needs to handle(Call `subscribe` to the event types). It reads a serialized event from the standard data, decodes it, and then updates the plugin's state accordingly, the return value is to indicate if a render is needed.

- `render`: Draw or refresh the plugin's output. The plugin's render method is invoked with the terminal size specified by rows and cols.

- `plugin_version`: Simply prints the plugin's version number.

Call `register_plugin` macro would export above functions to host automatically.

### How does zellij and plugin transfer the data
WASI has provided virtual file system for its `stdin`, `stdout` and `stderr`, e,g - [wasi_write_object](https://github.com/zellij-org/zellij/blob/main/zellij-server/src/plugins/zellij_exports.rs#L1069) writes the data to STDIN, and [wasi_read_object](https://github.com/zellij-org/zellij/blob/main/zellij-server/src/plugins/zellij_exports.rs#L1076) reads data from its STDOUT;
for plugin module, there are a few functions can be used:

   1. [object_from_stdin](https://github.com/zellij-org/zellij/blob/main/zellij-tile/src/shim.rs#L624)
   2. [bytes_from_stdin](https://github.com/zellij-org/zellij/blob/main/zellij-tile/src/shim.rs#L633)
   3. [object_to_stdout](https://github.com/zellij-org/zellij/blob/main/zellij-tile/src/shim.rs#L641)

### Protobuf support
Starting from version [0.38](https://github.com/zellij-org/zellij/releases/tag/v0.38.0), Zellij has incorporated Protocol Buffers (Protobuf) for more efficient and robust data serialization and transmission between the host and various modules. This allows complex data structures to be easily transferred across the system.

There are a bunches of APIs available under [zellij_utils crate](https://github.com/zellij-org/zellij/tree/main/zellij-utils/src/plugin_api).If you find that the existing APIs don't meet your requirements and you need to extend them, here's a step-by-step guide to doing so:
1. Create a New .proto File: The first step is to define your new API or message format. Create a new .proto file where you specify the message structure according to the Protobuf specification.
2. Compile the .proto File: Use the Protobuf compiler (protoc) to compile the .proto file. This will generate source code for encoding and decoding the message types in your .proto file.
3. Implement Conversion Code: Implement the necessary Rust code to convert between your custom types and the Protobuf-generated types. Add this code to the appropriate location.
4. Test Your Implementation: Before pushing your changes, make sure to thoroughly test the new API to ensure it works as expected and doesn't introduce any regressions.
