pub enum Colors {
    Default,
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
}

pub fn color_code(color: Colors) -> i32 {
    match color {
        Colors::Default => 0,
        Colors::Black => 30,
        Colors::White => 37,
        Colors::Red => 31,
        Colors::Green => 32,
        Colors::Blue => 34,
        Colors::Yellow => 33,
        Colors::Magenta => 35,
        Colors::Cyan => 36,
    }
}

pub fn bg_color_code(color: Colors) -> i32 {
    color_code(color) + 10
}
