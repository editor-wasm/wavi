use crate::die;
use crate::register_all;
use crate::Document;
use crate::Editor;
use once_cell::sync::Lazy;
use std::env;
use std::fs;
use std::io::Error;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

pub static mut EDITOR: Lazy<Editor> = Lazy::new(|| Editor::default());

#[allow(clippy::module_name_repetitions)]
pub struct Wavi {
    pub engine: Engine,
    pub linker: Linker<WasiCtx>,
    pub extensions: Vec<Extension>,
}

impl Wavi {
    pub fn new() -> Self {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
        register_all(&mut linker);

        let extensions = Extension::default(&engine, &mut linker);

        Wavi {
            engine: Engine::default(),
            linker,
            extensions,
        }
    }

    pub fn run(&mut self) {
        self.exec_on_loaded();
        loop {
            unsafe {
                if let Err(error) = EDITOR.refresh_screen() {
                    die(&error);
                }
                if EDITOR.should_quit {
                    break;
                }
                if let Err(error) = EDITOR.process_keypress() {
                    die(&error);
                }
            }
        }
    }

    fn exec_on_loaded(&mut self) {
        for extension in &mut self.extensions {
            let func = match self.linker.get(
                &mut extension.store,
                &extension.name,
                Some("editor_on_loaded"),
            ) {
                Some(Extern::Func(func)) => func,
                _ => panic!("failed load extension: {}", &extension.name),
            };

            func.typed::<(), (), _>(&mut extension.store)
                .expect("editor_on_loaded is failed. Please Check Params and Results.")
                .call(&mut extension.store, ())
                .expect("editor_on_loaded is failed.");
        }
    }
}

pub struct Extension {
    pub name: String,
    pub store: Store<WasiCtx>,
    pub module: Module,
}

impl Extension {
    pub fn default(engine: &Engine, linker: &mut wasmtime::Linker<WasiCtx>) -> Vec<Extension> {
        let mut result = Vec::new();
        let files = fs::read_dir("./extensions");

        if let Ok(files) = files {
            for file in files {
                let file = file.unwrap();
                let name = file.file_name().to_string_lossy().into_owned();
                let module = Module::from_file(&engine, file.path()).unwrap();
                let wasi = WasiCtxBuilder::new().inherit_stdio().build();
                let mut store = Store::new(&engine, wasi);

                linker.module(&mut store, &name, &module);
                result.push(Extension {
                    name,
                    store,
                    module,
                });
            }
            result
        } else {
            vec![]
        }
    }
}

// pub fn run() {
//     let engine = Engine::default();
//     let module = Module::from_file(&engine, "./extension/src/wasm/extension.wasm").unwrap();

//     let mut linker = Linker::new(&engine);
//     wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();
//     let wasi = WasiCtxBuilder::new().inherit_stdio().build();
//     let mut store = Store::new(&engine, wasi);
//     linker.module(&mut store, "", &module).unwrap();

//     linker
//         .get_default(&mut store, "")
//         .unwrap()
//         .typed::<(), (), _>(&store)
//         .unwrap()
//         .call(&mut store, ())
//         .unwrap();

//     // let instance = linker.instantiate(&mut store, &module).unwrap();
//     // let func = instance
//     //     .get_typed_func::<(), (), _>(&mut store, "main")
//     //     .unwrap();
//     // func.call(&mut store, ()).unwrap();
// }
