use bevy::asset::Handle;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Color, TextureAtlas, Transform};
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

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
    pub const TILE_ABOVE: f32 = 1.5;
    pub const VEGGIE: f32 = 2.;
    pub const VEG_UI: f32 = 3.;
    pub const COUNT_TEXT: f32 = 3.5;
    pub const VEG_DRAG: f32 = 4.;
}

pub fn collides(
    collider: Vec3,
    width: f32,
    height: f32,
    cursor_pos: Vec2,
) -> bool {
    (collider.x + width / 2. - cursor_pos.x / 2.).abs() < width / 2.
        && (collider.y + height / 2. - cursor_pos.y / 2.).abs() < height / 2.
}

pub fn text_mode_bundle(
    bg: Colors,
    fg: Colors,
    index: usize,
    x: f32,
    y: f32,
    z: f32,
    handle: Handle<TextureAtlas>,
) -> TextModeSpriteSheetBundle {
    TextModeSpriteSheetBundle {
        sprite: TextModeTextureAtlasSprite {
            bg: bg.get(),
            fg: fg.get(),
            index,
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        texture_atlas: handle,
        transform: Transform::from_xyz(x, y, z),
        ..Default::default()
    }
}