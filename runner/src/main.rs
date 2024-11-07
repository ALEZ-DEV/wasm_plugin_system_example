use std::fs::{self, read};

use wasmtime::{
    component::{Component, ComponentNamedList, Linker},
    Config, Engine, Store,
};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiView};

mod plugin;

#[tokio::main]
async fn main() {
    let wasm_file = read(concat!(env!("CARGO_MANIFEST_DIR"), "/../wasm/plugin.wasm")).unwrap();

    let engine =
        Engine::new(Config::new().async_support(true)).expect("Failed to create wasm engine");

    let component =
        Component::from_binary(&engine, &wasm_file).expect("Failed to compile wasm component");

    let mut linker = Linker::<WasmState>::new(&engine);
    wasmtime_wasi::add_to_linker_async(&mut linker).unwrap();

    let mut store = Store::new(
        &engine,
        WasmState {
            table: ResourceTable::new(),
            ctx: WasiCtx::builder().inherit_stdio().build(),
        },
    );

    let api = Api::instantiate_async(&mut store, &component, &linker)
        .await
        .expect("Failed to instantiate API component");

    let msg = Message {
        content: " world from the runner!".to_string(),
        name: "alez".to_string(),
    };

    api.call_init_plugin(&mut store)
        .await
        .expect("Failed to call 'init-plugin' function from wasm");

    api.call_greet(&mut store, &msg)
        .await
        .expect("Failed to call 'greet' function from wasm");
}

wasmtime::component::bindgen!({
    async: true,
    trappable_imports: true,
    path: "../api/wit",
});

struct WasmState {
    pub table: ResourceTable,
    ctx: wasmtime_wasi::WasiCtx,
}

impl WasiView for WasmState {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.ctx
    }
}
