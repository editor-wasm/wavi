use wasmtime::*;

pub fn registerAll(extension: Extension, editor: Editor) {
    register_insert(extension, editor)
}

pub fn register_insert(Extension: &mut Extension, Editor: &mut Editor) {
    Extension
        .linker
        .func_wrap("host", "editor_insert", |a: str| {
            Editor.document.insert(&Editor.cursor_position, c: char)
        })
}
