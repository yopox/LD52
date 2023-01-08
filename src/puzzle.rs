use bevy::utils::HashMap;
use strum::IntoEnumIterator;
use crate::veggie::Veggie;

pub struct Puzzle {
    pub size: (i8, i8),
    pub veggies: HashMap<Veggie, u8>,
    pub tiles: HashMap<(i8, i8), Tile>,
    pub placed: HashMap<(i8, i8), Veggie>,
}

impl Puzzle {
    pub fn remaining_veggie(&self, veggie: &Veggie) -> usize {
        if let Some(count) = self.veggies.get(veggie) {
            return *count as usize - self.placed.iter().filter(|(_, v)| **v == *veggie).count();
        }
        return 0;
    }
}

#[derive(Eq, PartialEq)]
pub enum Tile {
    Water,
    Rock,
    Scarecrow,
}

pub fn adjacent<A>(
    pos: (i8, i8),
    map: &HashMap<(i8, i8), A>,
) -> Vec<&A> {
    let mut adjacent = vec![];

    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            if dx == 0 && dy == 0 { continue; }

            if let Some(a) = map.get(&(pos.0 + dx, pos.1 + dy)) {
                adjacent.push(a);
            }
        }
    }

    adjacent
}

pub fn unhappy_adjacent(veggie: &Veggie) -> Vec<Veggie> {
    match veggie {
        Veggie::Garlic | Veggie::Carrot => vec![Veggie::Apple, Veggie::Mint],
        _ => vec![Veggie::Apple],
    }
}

pub fn is_happy(
    veggie: &Veggie,
    pos: (i8, i8),
    tiles: &HashMap<(i8, i8), Tile>,
    veggies: &HashMap<(i8, i8), Veggie>,
) -> (bool, bool) {
    let adjacent_veggies = adjacent(pos, veggies);
    let adjacent_tiles = adjacent(pos, tiles);

    // Unhappy caused by an adjacent veggie
    for impossible in unhappy_adjacent(veggie) {
        if adjacent_veggies.contains(&&impossible) { return (false, false); }
    }

    // Veggie specific conditions
    match veggie {
        Veggie::Strawberry => {
            (adjacent_veggies.contains(&&Veggie::Strawberry), true)
        }
        Veggie::Tomato => {
            (adjacent_veggies.contains(&&Veggie::Garlic)
            || adjacent_veggies.contains(&&Veggie::Carrot), true)
        }
        Veggie::Carrot => {
            (!adjacent_tiles.contains(&&Tile::Rock), true)
        }
        Veggie::Cherry => {
            // Exactly one adjacent cherry
            let c1 = adjacent_veggies.iter().filter(|&&v| v == &Veggie::Cherry).count() == 1;
            // No apples in the line / column
            let c2 = veggies.iter().filter(|(&(x, y), v)| v == &&Veggie::Apple && (x == pos.0 || y == pos.1)).count() == 0;
            (c1, c2)
        }
        Veggie::Garlic => {
            (!adjacent_tiles.contains(&&Tile::Water), true)
        }
        _ => { (true, true) }
    }
}

#[test]
fn test_strawberry() {
    let tiles = HashMap::from([]);
    let veggies = HashMap::from([((0, 0), Veggie::Strawberry)]);

    assert_eq!(is_happy(&Veggie::Strawberry, (1, 0), &tiles, &veggies), (true, true));
    assert_eq!(is_happy(&Veggie::Strawberry, (0, 2), &tiles, &veggies), (false, true));
}

#[test]
fn test_carrot() {
    let tiles = HashMap::from([((0, 0), Tile::Rock)]);
    let veggies = HashMap::from([]);

    assert_eq!(is_happy(&Veggie::Carrot, (0, 1), &tiles, &veggies), (false, true));
    assert_eq!(is_happy(&Veggie::Carrot, (0, 2), &tiles, &veggies), (true, true));
}

#[test]
fn test_garlic() {
    let tiles = HashMap::from([((0, 0), Tile::Water)]);
    let veggies = HashMap::from([]);

    assert_eq!(is_happy(&Veggie::Garlic, (0, 1), &tiles, &veggies), (false, true));
    assert_eq!(is_happy(&Veggie::Garlic, (0, 2), &tiles, &veggies), (true, true));
}

#[test]
fn test_apple() {
    let tiles = HashMap::from([]);
    let veggies = HashMap::from([((1, 1), Veggie::Apple)]);

    for veggie in Veggie::iter() {
        assert_eq!(is_happy(&veggie, (2, 1), &tiles, &veggies), (false, false));
    }
}

#[test]
fn test_mint() {
    let tiles = HashMap::from([]);
    let veggies = HashMap::from([((1, 1), Veggie::Mint)]);

    for veggie in [Veggie::Carrot, Veggie::Garlic] {
        assert_eq!(is_happy(&veggie, (2, 1), &tiles, &veggies), (false, false));
        assert_eq!(is_happy(&veggie, (3, 1), &tiles, &veggies), (true, true));
    }
}

#[test]
fn test_tomato() {
    let tiles = HashMap::from([]);
    let veggies = HashMap::from([((0, 0), Veggie::Garlic), ((5, 0), Veggie::Carrot)]);

    assert_eq!(is_happy(&Veggie::Tomato, (0, 5), &tiles, &veggies), (false, true));
    assert_eq!(is_happy(&Veggie::Tomato, (1, 0), &tiles, &veggies), (true, true));
    assert_eq!(is_happy(&Veggie::Tomato, (6, 0), &tiles, &veggies), (true, true));
}

#[test]
pub fn test_cherry() {
    let tiles = HashMap::from([]);
    let veggies = HashMap::from([((0, 0), Veggie::Apple), ((3, 1), Veggie::Cherry), ((1, 3), Veggie::Cherry)]);

    assert_eq!(is_happy(&Veggie::Cherry, (2, 1), &tiles, &veggies), (true, true));
    assert_eq!(is_happy(&Veggie::Cherry, (3, 3), &tiles, &veggies), (false, true));
    assert_eq!(is_happy(&Veggie::Cherry, (3, 0), &tiles, &veggies), (true, false));
    assert_eq!(is_happy(&Veggie::Cherry, (0, 3), &tiles, &veggies), (true, false));
    assert_eq!(is_happy(&Veggie::Cherry, (0, 5), &tiles, &veggies), (false, false));
}