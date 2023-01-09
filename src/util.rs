use bevy::asset::Handle;
use bevy::hierarchy::BuildChildren;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Color, Commands, ComputedVisibility, Entity, GlobalTransform, Res, TextureAtlas, Transform, Visibility};
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

use crate::loading::Textures;

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
    pub const VEG_UI_BG: f32 = 3.;
    pub const VEG_UI: f32 = 3.2;
    pub const COUNT_TEXT: f32 = 3.4;
    pub const VEG_DRAG: f32 = 4.;

    pub const TITLE_BUTTONS_BG: f32 = 1.;
    pub const TITLE_BUTTONS: f32 = 2.;
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
    bg: &Colors,
    fg: &Colors,
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

pub fn frame<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    textures: &Res<Textures>,
    x: f32, y: f32, z: f32,
    w: usize, h: usize,
    bg: Colors, fg: Colors,
) -> Entity {
    commands
        .spawn(Transform::from_xyz(x, y, z))
        .insert(Visibility::default())
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .with_children(|parent| {
            parent.spawn(TextModeSpriteSheetBundle {
                sprite: TextModeTextureAtlasSprite {
                    bg: fg.get(),
                    fg: bg.get(),
                    index: 0,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                transform: Transform {
                    scale: Vec3::new(w as f32, h as f32, 1.),
                    ..Default::default()
                },
                texture_atlas: textures.mrmotext.clone(),
                ..Default::default()
            });

            for (dx, dy, i) in [
                (0., 0., 7 * 32 + 30),
                (8. * w as f32 - 8., 0., 7 * 32 + 31),
                (0., h as f32 * 8. - 8., 6 * 32 + 30),
                (8. * w as f32 - 8., h as f32 * 8. - 8., 6 * 32 + 31),
            ] {
                parent
                    .spawn(text_mode_bundle(
                        &bg, &fg, i,
                        dx, dy, 0.01,
                        textures.mrmotext.clone()
                    ));
            }
        })
        .id()
}
