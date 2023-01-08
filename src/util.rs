use bevy::prelude::Color;

pub enum Colors {
    // PICO-8 palette
    Black,
    Navy,
    Beige,
    Red,
    Green,
    Orange,
    Grey,
    // Custom colors
    DarkBrown,
    Brown,
    DarkRed,
}

impl Colors {
    pub fn get(&self) -> Color {
        match self {
            Colors::Black => Color::BLACK,
            Colors::Navy => Color::hex("1d2b53").unwrap(),
            Colors::Beige => Color::hex("fff1e8").unwrap(),
            Colors::Red => Color::hex("ff004d").unwrap(),
            Colors::Green => Color::hex("008751").unwrap(),
            Colors::Orange => Color::hex("ffa300").unwrap(),
            Colors::Grey => Color::hex("c2c3c7").unwrap(),
            Colors::DarkBrown => Color::hex("441506").unwrap(),
            Colors::Brown => Color::hex("662916").unwrap(),
            Colors::DarkRed => Color::hex("553737").unwrap(),
        }
    }
}

pub mod z {
    pub const TILE: f32 = 1.;
    pub const VEGGIE: f32 = 2.;
    pub const VEG_UI: f32 = 3.;
    pub const COUNT_TEXT: f32 = 3.5;
    pub const VEG_DRAG: f32 = 4.;
}