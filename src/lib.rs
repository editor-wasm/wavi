#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::missing_inline_in_public_items
)]

mod document;
mod editor;
mod filetype;
mod highlighting;
mod hostFunctions;
mod row;
mod terminal;
mod wavi;

use document::Document;
use editor::{die, Editor, Position, SearchDirection};
use filetype::FileType;
use filetype::HighlightingOptions;
use row::Row;
use terminal::Terminal;

use hostFunctions::register_all;
use wavi::*;

pub fn run() {
    let mut wavi = Wavi::new();
    wavi.run();
}
