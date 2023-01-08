use bevy::prelude::Color;

pub enum Colors {
    Black,
    Navy,
    Beige,
    Red,
    Green,
    Orange,
    Grey,
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
        }
    }
}

pub mod z {
    pub const TILE: f32 = 1.;
    pub const VEGGIE: f32 = 2.;
    pub const VEG_UI: f32 = 3.;
    pub const VEG_DRAG: f32 = 4.;
}