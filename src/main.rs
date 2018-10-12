extern crate termion;

mod header;
mod render;

use header::headers_from_file;
use render::RenderState;
use std::io::{stdin, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        println!("Usage: '{} [filename]'", args.nth(0).unwrap());
        return;
    }

    let file = args.nth(1).unwrap();
    let headers = headers_from_file(&file).expect(&format!("File '{}' not found.", file));
    let mut window = RenderState::new(headers);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    window.print(&mut stdout);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Left => window.hide_children(),
            Key::Right => window.show_children(),
            Key::Up => window.select_up(),
            Key::Down => window.select_down(),
            Key::Char('[') => window.hide_all(),
            Key::Char(']') => window.show_all(),
            _ => {}
        };

        window.print(&mut stdout);
    }
}
