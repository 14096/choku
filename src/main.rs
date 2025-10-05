extern crate choku;

use choku::utils::colors::Colors::*;
use choku::{Choku, Color};

fn main() {
    let red_yellow = Choku::new(Color::Default(Red), "Hello, World!")
        .underline()
        .bg(Cyan)
        .bold()
        .color();

    let custom_rgb = Choku::new(Color::RGB(0, 230, 0), "Custom RGB Color")
        .italic()
        .color();

    println!("{}", red_yellow);
    println!("{}", custom_rgb);
}
