use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::loading::Textures;

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
}

pub fn spawn_veggie<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    textures: &Res<Textures>,
    position: Vec3,
    veggie: &Veggie,
) -> EntityCommands<'w, 's, 'a> {
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
}