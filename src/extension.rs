use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

pub fn run() {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "./extension/src/wasm/extension.wasm").unwrap();

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi);
    linker.module(&mut store, "", &module).unwrap();

    linker
        .get_default(&mut store, "")
        .unwrap()
        .typed::<(), (), _>(&store)
        .unwrap()
        .call(&mut store, ())
        .unwrap();
}
