use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use rand::random;
use strum::EnumIter;
use crate::loading::Textures;
use crate::util::Colors;

#[derive(Eq, PartialEq, EnumIter)]
pub enum Veggie {
    Strawberry,
    Tomato,
    Apple,
    Carrot, // Littéralement Clément
    Cherry,
    Garlic,
    Mint,
}

impl Veggie {
    pub fn sprite(&self) -> usize {
        match self {
            Veggie::Strawberry => 0,
            Veggie::Tomato => 1,
            Veggie::Apple => 2,
            Veggie::Carrot => 3,
            Veggie::Cherry => 4,
            Veggie::Garlic => 5,
            Veggie::Mint => 6,
        }
    }

    pub fn faces(&self) -> Vec<(f32, f32)> {
        match self {
            Veggie::Strawberry | Veggie::Apple => vec![(16., 16.)],
            Veggie::Tomato => vec![(16., 12.)],
            Veggie::Carrot => vec![(16., 20.)],
            Veggie::Cherry => vec![(8., 7.), (24., 7.)],
            Veggie::Garlic => vec![(16., 14.)],
            Veggie::Mint => vec![(8., 25.)],
        }
    }

    pub fn face_color(&self) -> Color {
        match self {
            Veggie::Strawberry | Veggie::Tomato | Veggie::Cherry => Colors::Red.get(),
            Veggie::Apple | Veggie::Mint => Colors::Green.get(),
            Veggie::Carrot => Colors::Orange.get(),
            Veggie::Garlic => Colors::Grey.get()
        }
    }
}

#[derive(Component)]
struct Face;

pub enum Expression {
    Neutral,
    Surprised,
    Happy,
    Sad,
}

impl Expression {
    fn index(&self) -> usize {
        match self {
            Expression::Neutral => 0,
            Expression::Surprised => 1,
            Expression::Sad => 2,
            Expression::Happy => 3,
        }
    }
}

pub fn spawn_veggie<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    textures: &Res<Textures>,
    position: Vec3,
    veggie: &Veggie,
    expression: Expression,
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
                            index: expression.index(),
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