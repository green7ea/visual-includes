extern crate termion;
extern crate std;

use header::Header;
use std::io::Write;

pub struct RenderState {
    headers: Vec<Header>,
    selected: usize,
    skip_lines: usize,
}

impl RenderState {
    pub fn new(headers: Vec<Header>) -> RenderState {
        RenderState {
            headers,
            selected: 0,
            skip_lines: 0,
        }
    }

    pub fn print(&self, stdout: &mut termion::raw::RawTerminal<std::io::Stdout>) {

        let height = termion::terminal_size().unwrap().1;

        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        ).unwrap();

        self.headers
            .iter()
            .enumerate()
            .filter(|(_, header)| header.visible)
            .map(|(i, header)| (self.selected == i, header))
            .skip(self.skip_lines)
            .take(height as usize)
            .enumerate()
            .for_each(|(line, (selected, header))| {
                // We want to start at 1 not 0
                let line = line + 1;

                if selected {
                    write!(stdout, "{}*", termion::cursor::Goto(1, line as u16),);
                }

                let indent = ((header.depth * 2) + 1) as u16;
                write!(
                    stdout,
                    "{}{} ({})",
                    termion::cursor::Goto(indent, line as u16),
                    header.path,
                    header.dependencies
                );
            });

        stdout.flush().unwrap();
    }

    pub fn select_up(&mut self) {
        self.selected = self
            .headers
            .iter()
            .enumerate()
            .take_while(|(i, _)| i < &self.selected)
            .filter(|(_, header)| header.visible)
            .last()
            .map(|(i, _)| i)
            .unwrap_or(self.selected)
    }

    pub fn select_down(&mut self) {
        self.selected = self
            .headers
            .iter()
            .enumerate()
            .skip_while(|(i, _)| i <= &self.selected)
            .filter(|(_, header)| header.visible)
            .nth(0)
            .map(|(i, _)| i)
            .unwrap_or(self.selected)
    }

    pub fn show_children(&mut self) {
        let depth = self.headers[self.selected].depth;
        let selected = self.selected;

        self.headers
            .iter_mut()
            .enumerate()
            .skip_while(|(i, _)| i <= &selected)
            .take_while(|(_, header)| header.depth > depth)
            .for_each(|(_, header)| header.visible = header.depth == depth + 1);
    }

    pub fn hide_children(&mut self) {
        let depth = self.headers[self.selected].depth;
        let selected = self.selected;

        self.headers
            .iter_mut()
            .enumerate()
            .skip_while(|(i, _)| i <= &selected)
            .take_while(|(_, header)| header.depth > depth)
            .for_each(|(_, header)| header.visible = false);
    }

    pub fn show_all(&mut self) {
        self.headers
            .iter_mut()
            .for_each(|header| header.visible = true);
    }

    pub fn hide_all(&mut self) {
        self.headers
            .iter_mut()
            .for_each(|header| header.visible = header.depth == 1);
    }
}
