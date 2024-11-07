# Plugin system for Rust with WebAssembly
## This is a proof of concept!

This repo is an proof of concept (or simply an example) on how you would be able to implement a plugin system with WebAssembly for your rust app

This repo was started because of the interesting [Zed blogpost](https://zed.dev/blog/zed-decoded-extensions) and the code is heavily inspired of the [zed_extension_api crate](https://github.com/zed-industries/zed/tree/main/crates/extension_api), so thanks to them  
Don't hesitate to use the code as you want

> [!WARNING]
> PLS take what I say with a pinch of salt, I'm not an expert on the subject. 
> This was only an exploration on the subject for my owns projects

### Architecture

The architecture of the system is really simple, it's separated in 3 parts :
- Runner
  - This is the native part of the app, which will run our Wasm compiled code
- API (or you could say library or SDK or whatever, I don't really care)
  - This is where you will implement your API, all the methods and types that the plugin dev can use will be there. This part can theoretically be implemented in any language, all you need is the wit file and check if [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) can generate the code for the concerned language
- Plugin
  - This part will be implemented by the user of your library (The API part), this one can theoretically be implemented in any language too

### WIT file

WIT stand for Wasm Interface Type, put it simply, this is your interface where you will describe your API

In your wit file you will need to add a "world", it's a set of imports and exports
- Imported function
  - This will be the method that you can implement in your runner, the Wasm user will be able to call it afterward in is plugin code
- Exported function
  - This is the method which will be implemented by the Wasm user, you will be able to call them to execute custom code

I think I needed to clarify this part because the docs is NOT CLEAR at all and a bit outdated

### Note

I won't go further in the details, because I think that exploring the code is the best way to learn

If this repo is outdated, don't hesitate to open a issue or make a PR to update it

### Source

There's some source to help you understand the code

[WIT Documentation](https://component-model.bytecodealliance.org/design/wit.html#worlds)
[Wasmtime book](https://docs.wasmtime.dev/)
[Wasmtime rust doc](https://docs.rs/wasmtime/latest/wasmtime/)
[Zed blogpost](https://zed.dev/blog/zed-decoded-extensions)
[Zed wasm crate](https://github.com/zed-industries/zed/tree/main/crates/extension_api)

