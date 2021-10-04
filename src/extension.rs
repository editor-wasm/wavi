use std::env;
use std::fs;
use std::io::Error;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

pub struct Extensions<T> {
    pub engine: Engine,
    pub extensions: Vec<Extension<T>>,
}

impl Extensions<wasmtime_wasi::WasiCtx> {
    pub fn default() -> Self {
        let engine = Engine::default();
        let extensions = Extension::default(&engine);

        Extensions {
            engine: Engine::default(),
            extensions,
        }
    }
}

pub struct Extension<T> {
    pub store: Store<T>,
    pub module: Module,
    pub linker: Linker<T>,
}

impl Extension<wasmtime_wasi::WasiCtx> {
    pub fn default(engine: &Engine) -> Vec<Extension<wasmtime_wasi::WasiCtx>> {
        let mut result = Vec::new();
        let files = fs::read_dir("wavi/src/extension.rs");

        if let Ok(files) = files {
            for file in files {
                let file = file.unwrap();

                let module = Module::from_file(&engine, file.path()).unwrap();
                let mut linker = Linker::new(&engine);
                // wasi setup
                wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
                let wasi = WasiCtxBuilder::new().inherit_stdio().build();
                let mut store = Store::new(&engine, wasi);
                result.push(Extension {
                    store,
                    module,
                    linker,
                });
            }
            result
        } else {
            vec![]
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
