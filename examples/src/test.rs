extern crate rustbox;

use std::char;

use rustbox::{Style,Color};

fn main() {
    rustbox::init();
    rustbox::print(1, 1, Style::Bold, Color::White, Color::Black, "Hello, world!");
    rustbox::print(1, 3, Style::Bold, Color::White, Color::Black, "Press 'q' to quit.");
    rustbox::present();

    loop {
        match rustbox::poll_event() {
            rustbox::Event::KeyEvent(_, _, ch) => {
                match char::from_u32(ch) {
                    Some('q') => { break; },
                    _ => {}
                }
            },
            _ => { }
        }
    }
    rustbox::shutdown();
}
