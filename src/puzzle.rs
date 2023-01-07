use bevy::utils::HashMap;
use crate::veggie::Veggie;

pub struct Puzzle {
    pub size: (i8, i8),
    pub veggies: Vec<(Veggie, u8)>,
    pub tiles: HashMap<(i8, i8), Tile>,
}

pub enum Tile {
    Water,
    Rock,
    Scarecrow,
}

pub fn is_happy(
    veggie: &Veggie,
    pos: (i8, i8),
    tiles: &HashMap<(i8, i8), Tile>,
    veggies: &HashMap<(i8, i8), Veggie>,
) -> bool {
    match veggie {
        Veggie::Strawberry => { true }
        Veggie::Tomato => { true }
        Veggie::Apple => {
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if dx == 0 && dy == 0 { continue }
                    if veggies.contains_key(&(pos.0 + dx, pos.1 + dy)) { return false }
                }
            }
            return true
        }
        Veggie::Carrot => { true }
        Veggie::Cherry => { true }
        Veggie::Garlic => { true }
        Veggie::Mint => { true }
    }
}