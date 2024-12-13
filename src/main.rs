#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
mod editor;
use crate::editor::Editor;


fn main() {
    Editor::default().run();
}