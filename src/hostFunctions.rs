use crate::wavi::EDITOR;
use crate::Wavi;
use std::str;
use termion::event::Key;
use wasmtime::Linker;
use wasmtime::{Caller, Extern};
use wasmtime_wasi::WasiCtx;

pub fn register_all(linker: &mut Linker<WasiCtx>) {
    register_insert(linker);
}

fn register_insert(linker: &mut Linker<WasiCtx>) {
    #[allow(clippy::unwrap_used)]
    linker
        .func_wrap(
            "env",
            "editor_insert",
            |mut caller: Caller<'_, _>, ptr: u32, len: u32| {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => panic!("failed to find host memory"),
                };
                let mut data = vec![0_u8; len as usize];
                // let data = mem
                //     .data(&caller)
                //     .get(ptr as u32 as usize..)
                //     .and_then(|arr| arr.get(..len as u32 as usize));
                mem.read(&caller, ptr as usize, &mut data);
                let string = match str::from_utf8(&data) {
                    Ok(s) => s,
                    Err(_) => panic!("invalid utf-8"),
                };
                for c in string.chars() {
                    unsafe {
                        EDITOR.document.insert(&EDITOR.cursor_position, c);
                        EDITOR.move_cursor(Key::Right);
                    }
                }
            },
        )
        .unwrap();
}
