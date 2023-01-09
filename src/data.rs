use bevy::log::error;
use bevy::utils::HashMap;
use crate::puzzle;
use crate::puzzle::{Puzzle, Tile};
use crate::veggie::Veggie;

/// Encode puzzle
/// - [a-z] x9-  -> 5b (00000 = stop, 00001 = 'a', etc)
/// - width      -> 5b
/// - height     -> 4b
/// - tiles/vegs -> 13b (x 5b + y 4b + 0000=rock, 0001=water, 0010=strawberry, etc)

pub struct Encoder;

impl Encoder {
    pub fn encode_puzzle(puzzle: &Puzzle) -> Option<String> {

        match puzzle.is_valid() {
            Err(e) => {
                #[cfg(target_arch = "wasm32")]
                if let Some(window) = web_sys::window() {
                    window.alert_with_message(&format!("Can't export level: {}", e));
                }
                return None;
            },
            _ => {},
        }

        let mut data = Vec::new();
        for (i, c) in puzzle.author.chars().enumerate() {
            if i > 9 { continue; }
            data.append(&mut Encoder::encode_u5((c as u32 - 'a' as u32 + 1) as u8));
        }
        data.append(&mut vec![false, false, false, false, false]);
        data.append(&mut Encoder::encode_u5(puzzle.size.0 as u8));
        data.append(&mut Encoder::encode_u4(puzzle.size.1 as u8));

        for (&(x, y), tile) in puzzle.tiles.iter() {
            if x >= 0 && y >= 0 && x < puzzle::MAX_W && y < puzzle::MAX_H {
                data.append(&mut Encoder::encode_u5(x as u8));
                data.append(&mut Encoder::encode_u4(y as u8));
                data.append(&mut Encoder::encode_tile(tile));
            }
        }

        for (&(x, y), veg) in puzzle.placed.iter() {
            if x >= 0 && y >= 0 && x < puzzle::MAX_W && y < puzzle::MAX_H {
                data.append(&mut Encoder::encode_u5(x as u8));
                data.append(&mut Encoder::encode_u4(y as u8));
                data.append(&mut Encoder::encode_veggie(veg));
            }
        }

        while data.len() % 8 != 0 {
            data.push(false);
        }

        let data_u8 = data
            .chunks(8)
            .map(|c| Encoder::slice_to_u8(c))
            .collect::<Vec<u8>>();

        let encoded = base91::slice_encode(&data_u8);
        return Some(String::from_utf8_lossy(&encoded).to_string());
    }

    fn encode_u5(n: u8) -> Vec<bool> {
        let mut vec = Vec::new();
        vec.push(((n & 0b00010000) >> 4) == 1);
        vec.push(((n & 0b00001000) >> 3) == 1);
        vec.push(((n & 0b00000100) >> 2) == 1);
        vec.push(((n & 0b00000010) >> 1) == 1);
        vec.push( (n & 0b00000001) == 1);
        return vec;
    }

    fn encode_u4(n: u8) -> Vec<bool> {
        let mut vec = Vec::new();
        vec.push(((n & 0b00001000) >> 3) == 1);
        vec.push(((n & 0b00000100) >> 2) == 1);
        vec.push(((n & 0b00000010) >> 1) == 1);
        vec.push( (n & 0b00000001) == 1);
        return vec;
    }

    fn encode_tile(tile: &Tile) -> Vec<bool> {
        match *tile {
            Tile::Water => vec![false, false, false, true],
            Tile::Rock => vec![false, false, true, false],
        }
    }

    fn encode_veggie(veggie: &Veggie) -> Vec<bool> {
        match *veggie {
            Veggie::Strawberry => vec![false, false, true, true],
            Veggie::Tomato => vec![false, true, false, false],
            Veggie::Apple => vec![false, true, false, true],
            Veggie::Carrot => vec![false, true, true, false],
            Veggie::Cherry => vec![false, true, true, true],
            Veggie::Garlic => vec![true, false, false, false],
            Veggie::Mint => vec![true, false, false, true],
        }
    }

    fn slice_to_u8(p0: &[bool]) -> u8 {
        let mut result = 0;
        for (i, b) in p0.iter().enumerate() {
            if *b { result += 1 << (7 - i); }
        }
        return result;
    }
}

pub struct Decoder;

impl Decoder {
    pub fn decode_puzzle(str: String) -> Option<Puzzle> {
        let decoded = base91::slice_decode(str.as_bytes());

        let mut bits = decoded.iter().flat_map(|n| Decoder::u8_to_slice(*n)).collect::<Vec<bool>>();

        let mut author = String::new();
        loop {
            if bits.len() < 5 { return None; }
            let char = bits.drain(0..5).collect::<Vec<bool>>();
            if char.iter().all(|b| !*b) { break; }
            author.push(Decoder::decode_char(&char));
        }

        let width = Decoder::decode_u5(&bits.drain(0..5).collect::<Vec<bool>>());
        let height = Decoder::decode_u4(&bits.drain(0..4).collect::<Vec<bool>>());

        let mut tiles = HashMap::new();
        let mut placed = HashMap::new();

        loop {
            if bits.len() < 13 { break; }
            let x = Decoder::decode_u5(&bits.drain(0..5).collect::<Vec<bool>>());
            let y = Decoder::decode_u4(&bits.drain(0..4).collect::<Vec<bool>>());
            let key = (x as i8, y as i8);
            match Decoder::decode_u4(&bits.drain(0..4).collect::<Vec<bool>>()) {
                1 => { tiles.insert(key, Tile::Water); },
                2 => { tiles.insert(key, Tile::Rock); },
                3 => { placed.insert(key, Veggie::Strawberry); },
                4 => { placed.insert(key, Veggie::Tomato); },
                5 => { placed.insert(key, Veggie::Apple); },
                6 => { placed.insert(key, Veggie::Carrot); },
                7 => { placed.insert(key, Veggie::Cherry); },
                8 => { placed.insert(key, Veggie::Garlic); },
                9 => { placed.insert(key, Veggie::Mint); },
                _ => { error!("Couldn't parse tile/veg :("); return None; },
            }
        }

        let mut puzzle = Puzzle {
            author,
            size: (width as i8, height as i8),
            veggies: HashMap::new(),
            tiles,
            placed: placed.clone(),
        };

        match puzzle.is_valid() {
            Ok(_) => {
                for (_, veg) in placed.iter() {
                    if puzzle.veggies.contains_key(veg) {
                        *puzzle.veggies.get_mut(veg).unwrap() += 1;
                    } else {
                        puzzle.veggies.insert(*veg, 1);
                    }
                }
                return Some(puzzle);
            }
            Err(e) => error!("Invalid puzzle: {}", e),
        }

        return None;
    }

    fn u8_to_slice(n: u8) -> Vec<bool> {
        let mut result = vec![];
        result.push(((n & 0b10000000) >> 7) == 1);
        result.push(((n & 0b01000000) >> 6) == 1);
        result.push(((n & 0b00100000) >> 5) == 1);
        result.push(((n & 0b00010000) >> 4) == 1);
        result.push(((n & 0b00001000) >> 3) == 1);
        result.push(((n & 0b00000100) >> 2) == 1);
        result.push(((n & 0b00000010) >> 1) == 1);
        result.push( (n & 0b00000001) == 1);
        return result;
    }

    fn decode_u5(slice: &[bool]) -> u8 {
        let mut result = 0;
        if slice[0] { result += 0b00010000; }
        if slice[1] { result += 0b00001000; }
        if slice[2] { result += 0b00000100; }
        if slice[3] { result += 0b00000010; }
        if slice[4] { result += 0b00000001; }
        return result;
    }

    fn decode_u4(slice: &[bool]) -> u8 {
        let mut result = 0;
        if slice[0] { result += 0b00001000; }
        if slice[1] { result += 0b00000100; }
        if slice[2] { result += 0b00000010; }
        if slice[3] { result += 0b00000001; }
        return result;
    }

    fn decode_char(slice: &[bool]) -> char {
        return (Decoder::decode_u5(slice) - 1 + 'a' as u8) as char;
    }
}