use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

pub struct Extensions<T> {
    pub engine: Engine,
    pub extensions: Vec<Extension<T>>,
}

pub struct Extension<T> {
    pub store: Store<T>,
    pub module: Module,
    pub linker: Linker<T>,
}

impl Extension<wasmtime_wasi::WasiCtx> {
    pub fn default(engine: Engine, name: &str) -> Self {
        let module =
            Module::from_file(&engine, format!("./extension/src/wasm/{}.wasm", name)).unwrap();

        let mut linker = Linker::new(&engine);
        // wasi setup
        wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
        let wasi = WasiCtxBuilder::new().inherit_stdio().build();

        let mut store = Store::new(&engine, wasi);

        Extension {
            store,
            module,
            linker,
        }
    }
}

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

    // let instance = linker.instantiate(&mut store, &module).unwrap();
    // let func = instance
    //     .get_typed_func::<(), (), _>(&mut store, "main")
    //     .unwrap();
    // func.call(&mut store, ()).unwrap();
}
