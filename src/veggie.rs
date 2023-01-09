use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::TextModeTextureAtlasSprite;
use strum::EnumIter;
use crate::loading::Textures;
use crate::util::{Colors, text_mode_bundle};

#[derive(Eq, PartialEq, Copy, Clone, Debug, EnumIter, Hash, Component)]
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

    pub fn face_bg(&self) -> Colors {
        match self {
            Veggie::Strawberry | Veggie::Tomato | Veggie::Cherry => Colors::Red,
            Veggie::Apple | Veggie::Mint => Colors::Green,
            Veggie::Carrot => Colors::Orange,
            Veggie::Garlic => Colors::Grey
        }
    }

    pub fn face_fg(&self) -> Colors {
        match self {
            Veggie::Carrot | Veggie::Garlic => Colors::Black,
            _ => Colors::Beige
        }
    }
}

pub struct VeggiePlugin;

impl Plugin for VeggiePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UpdateFaces>()
            .add_system(update_faces);
    }
}

pub struct UpdateFaces(pub Entity, pub (Expression, Expression));

fn update_faces(
    mut events: EventReader<UpdateFaces>,
    veg_children: Query<&Children, With<Veggie>>,
    mut faces: Query<&mut TextModeTextureAtlasSprite, With<Face>>,
) {
    for UpdateFaces(e, (e1, e2)) in events.iter() {
        if let Ok(c) = veg_children.get(*e) {
            if let Some(f1) = c.get(0) {
                let mut sprite = faces.get_mut(*f1).unwrap();
                sprite.index = e1.index();
            }

            if let Some(f2) = c.get(1) {
                let mut sprite = faces.get_mut(*f2).unwrap();
                sprite.index = e2.index();
            }
        }
    }
}

#[derive(Component)]
pub struct Face;

#[derive(Debug)]
pub enum Expression {
    Neutral,
    Surprised,
    Happy,
    Sad,
}

impl Expression {
    pub(crate) fn index(&self) -> usize {
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
        .insert(*veggie)
        .with_children(|parent| {
            veggie.faces().iter().for_each(|(x, y)| {
                parent
                    .spawn(text_mode_bundle(
                        veggie.face_bg(),
                        veggie.face_fg(),
                        expression.index(),
                        *x, *y, 0.0000001,
                        textures.faces.clone()
                    ))
                    .insert(Face);
            });
        })
        .id()
}