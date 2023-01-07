use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use rand::random;
use crate::loading::Textures;
use crate::util::Colors;

pub enum Veggie {
    Strawberry,
    Tomato,
    Apple,
    Carrot,
    Cherry,
}

impl Veggie {
    pub fn sprite(&self) -> usize {
        match self {
            Veggie::Strawberry => 0,
            Veggie::Tomato => 1,
            Veggie::Apple => 2,
            Veggie::Carrot => 3,
            Veggie::Cherry => 4,
        }
    }

    pub fn faces(&self) -> Vec<(f32, f32)> {
        match self {
            Veggie::Strawberry => vec![(16., 16.)],
            Veggie::Tomato => vec![(16., 12.)],
            Veggie::Apple => vec![(16., 16.)],
            Veggie::Carrot => vec![(16., 20.)],
            Veggie::Cherry => vec![(8., 7.), (24., 7.)],
        }
    }

    pub fn face_color(&self) -> Color {
        match self {
            Veggie::Strawberry | Veggie::Tomato | Veggie::Cherry => Colors::Red.get(),
            Veggie::Apple => Colors::Green.get(),
            Veggie::Carrot => Colors::Orange.get(),
        }
    }
}

#[derive(Component)]
struct Face;

pub fn spawn_veggie<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    textures: &Res<Textures>,
    position: Vec3,
    veggie: &Veggie,
) -> Entity {
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: veggie.sprite(),
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture_atlas: textures.fruit.clone(),
            transform: Transform {
                translation: position,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            veggie.faces().iter().for_each(|(x, y)| {
                parent
                    .spawn(TextModeSpriteSheetBundle {
                        sprite: TextModeTextureAtlasSprite {
                            bg: veggie.face_color(),
                            fg: Colors::Beige.get(),
                            index: (random::<f32>() * 4.) as usize,
                            anchor: Anchor::BottomLeft,
                            ..Default::default()
                        },
                        texture_atlas: textures.faces.clone(),
                        transform: Transform::from_xyz(*x, *y, 0.1),
                        ..Default::default()
                    })
                    .insert(Face);
            });
        })
        .id()
}