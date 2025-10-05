use std::fmt::Display;
use utils::{colors::Colors, styles::Styles};

pub mod utils {
    pub mod colors;
    pub mod styles;
}

pub enum Color {
    Default(Colors),
    RGB(u8, u8, u8),
}

pub struct Choku {
    string: String,
}

impl Choku {
    pub fn new<'a>(color: Color, string: &'a str) -> Choku {
        let choku = Choku {
            string: match color {
                Color::Default(c) => format_default(utils::colors::color_code(c), string),
                Color::RGB(r, g, b) => format_rgb(r, g, b, string),
            },
        };

        choku
    }

    pub fn color(&mut self) -> String {
        self.string.to_string()
    }

    pub fn bg(&mut self, color: Colors) -> &mut Choku {
        self.string = format_default(utils::colors::bg_color_code(color), &self.string);
        self
    }

    pub fn bold(&mut self) -> &mut Choku {
        self.string = Choku::style(Styles::Bold, &self.string);
        self
    }

    pub fn italic(&mut self) -> &mut Choku {
        self.string = Choku::style(Styles::Italic, &self.string);
        self
    }

    pub fn underline(&mut self) -> &mut Choku {
        self.string = Choku::style(Styles::Underline, &self.string);
        self
    }

    pub fn dim(&mut self) -> &mut Choku {
        self.string = Choku::style(Styles::Dim, &self.string);
        self
    }

    pub fn slow_blink(&mut self) -> &mut Choku {
        self.string = Choku::style(Styles::SlowBlink, &self.string);
        self
    }

    pub fn fast_blink(&mut self) -> &mut Choku {
        self.string = Choku::style(Styles::FastBlink, &self.string);
        self
    }

    fn style(style: Styles, string: &String) -> String {
        let style_code = utils::styles::style_code(style);
        format_default(style_code, string)
    }
}

fn format_default<T: Display>(code: i32, data: T) -> String {
    format!("\x1B[{}m{}\x1B[0m", code, data)
}

fn format_rgb(r: u8, g: u8, b: u8, data: &str) -> String {
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, data)
}
