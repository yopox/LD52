use bevy::utils::HashMap;
use crate::veggie::Veggie;

pub struct Puzzle {
    pub size: (u8, u8),
    pub veggies: Vec<(Veggie, u8)>,
    pub tiles: HashMap<(u8, u8), Tile>,
}

pub enum Tile {
    Water,
    Rock,
    Scarecrow,
}