use std::u8;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::random;
use strum::IntoEnumIterator;

use crate::{data, GameState, HEIGHT, util, WIDTH};
use crate::audio::{BGM, PlayBgmEvent};
use crate::data::Decoder;
use crate::grid::CurrentPuzzle;
use crate::loading::Textures;
use crate::puzzle::Puzzle;
use crate::text::{ButtonClick, spawn_text, TextButtonId};
use crate::util::Colors;
use crate::veggie::{Expression, spawn_veggie, Veggie};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Title).with_system(setup))
            .add_system_set(SystemSet::on_resume(GameState::Title).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Title).with_system(update))
            .add_system_set(SystemSet::on_exit(GameState::Title).with_system(cleanup))
            .add_system_set(SystemSet::on_pause(GameState::Title).with_system(cleanup))
        ;
    }
}

#[derive(Component)]
struct TitleUI;

fn get_combination(n: u8) -> Vec<(Veggie, f32, f32, Expression)> {
    match n % 6 {
        5 => vec![
            (Veggie::Carrot, 313., 280., Expression::Sad),
            (Veggie::Mint, 348., 280., Expression::Happy),
        ],
        4 => vec![
            (Veggie::Mint, 312., 280., Expression::Happy),
            (Veggie::Garlic, 348., 278., Expression::Sad),
        ],
        3 => vec![
            (Veggie::Carrot, 167., 280., Expression::Neutral),
            (Veggie::Carrot, 313., 280., Expression::Neutral),
            (Veggie::Carrot, 348., 280., Expression::Neutral),
            (Veggie::Carrot, 378., 280., Expression::Happy),
            (Veggie::Carrot, 446., 280., Expression::Neutral),
        ],
        2 => vec![
            (Veggie::Tomato, 167., 280., Expression::Sad),
            (Veggie::Garlic, 395., 294., Expression::Happy),
        ],
        1 => vec![
            (Veggie::Cherry, 167., 279., Expression::Sad),
            (Veggie::Carrot, 313., 280., Expression::Sad),
            (Veggie::Mint, 352., 280., Expression::Surprised),
            (Veggie::Apple, 446., 280., Expression::Happy),
        ],
        _ => vec![
            (Veggie::Strawberry, 243., 296., Expression::Sad),
            (Veggie::Apple, 348., 280., Expression::Surprised),
            (Veggie::Carrot, 446., 280., Expression::Happy),
        ],
    }
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    mut bgm: EventWriter<PlayBgmEvent>,
) {
    bgm.send(PlayBgmEvent(BGM::Title));

    // Title
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(WIDTH / 2., HEIGHT * 3. / 4., 0.),
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            texture: textures.title.clone(),
            ..Default::default()
        })
        .insert(TitleUI);

    // Veggies
    for (v, x, y, e) in get_combination((random::<f32>() * 6.) as u8) {
        let id = spawn_veggie(
            &mut commands,
            &textures,
            Vec3::new(x, y, util::z::VEGGIE),
            &v,
            e,
        );

        commands
            .entity(id)
            .insert(TitleUI);
    }

    // Buttons frame
    let id = util::frame(
        &mut commands, &textures,
        WIDTH / 2. - 8. * 7.5, 116., util::z::TITLE_BUTTONS_BG,
        17, 13,
        Colors::DarkRed, Colors::Beige
    );

    commands
        .entity(id)
        .insert(TitleUI);

    // Buttons
    #[cfg(target_arch = "wasm32")]
        let load = "---load------\n-----level---";
    #[cfg(not(target_arch = "wasm32"))]
        let load = "--load-from--\n--clipboard--";

    for (text, x, y, button) in [
        ("---level-----\n------list---".to_string(), WIDTH / 2. - 8. * 5.5, 184. + 16., TextButtonId::Title(0)),
        (load.to_string(), WIDTH / 2. - 8. * 5.5, 184. - 16., TextButtonId::Title(1)),
        ("---level-----\n----editor---".to_string(), WIDTH / 2. - 8. * 5.5, 184. - 48., TextButtonId::Title(2)),
    ] {
        let id = spawn_text(
            &mut commands,
            &textures,
            Vec3::new(x, y, util::z::TITLE_BUTTONS),
            &text,
            Colors::Beige,
            Colors::DarkRed,
        );

        commands
            .entity(id)
            .insert(button)
            .insert(TitleUI);
    }

    // All veggies
    for (i, v) in Veggie::iter().enumerate() {
        let id = spawn_veggie(
            &mut commands,
            &textures,
            Vec3::new( (WIDTH - Veggie::iter().len() as f32 * 50.) / 2. + i as f32 * 50.,
                       60. + match v { Veggie::Cherry => -1., Veggie::Garlic => -2., _ => 0. },
                       util::z::VEGGIE),
            &v,
            match (random::<f32>() * 8.0) as u8 {
                0 | 1 => Expression::Happy,
                2 => Expression::Surprised,
                3 => Expression::Neutral,
                _ => Expression::Sad,
            }
        );

        commands
            .entity(id)
            .insert(TitleUI);
    }

    // Partial grid
    let w = (WIDTH - 10. * 40.) / 2.;
    for y in 0..2 {
        for x in 0..10 {
            let tile_x = w + x as f32 * 40.;
            let tile_y = -20. + y as f32 * 40.;

            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: ((x + y) % 2) as usize,
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(tile_x, tile_y, util::z::TILE),
                    texture_atlas: textures.tile.clone(),
                    ..Default::default()
                })
                .insert(TitleUI);
            }
        }

    // Corners
    for (dx, dy, fx, fy, dx2, dy2, sx, sy, i) in [
        (-8., 2. * 40., false, false, 8., 0., 10. * 5., 1., 1),
        (10. * 40., 2. * 40., true, false, 0., -1. * 2. * 40., 1., 2. * 5., 2),
        (-8., -8., false, true, 8., 0., 10. * 5., 1., 3),
        (10. * 40., -8., true, true, -1. * 10. * 40. - 8., 8., 1., 2. * 5., 4),
    ] {
        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    flip_x: fx,
                    flip_y: fy,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                transform: Transform::from_xyz(w + dx, -20. + dy, util::z::TILE),
                texture_atlas: textures.border.clone(),
                ..Default::default()
            })
            .insert(TitleUI);

        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: i,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(w + dx + dx2, -20. + dy + dy2, util::z::TILE),
                    scale: Vec3::new(sx, sy, 1.),
                    ..Default::default()
                },
                texture_atlas: textures.border.clone(),
                ..Default::default()
            })
            .insert(TitleUI);
    }

    let id = spawn_text(
        &mut commands,
        &textures,
        Vec3::new(8., 24., util::z::TITLE_BUTTONS),
        "a game by\nyopox &\njmen_balec",
        Colors::DarkRed,
        Colors::Beige,
    );
    commands.entity(id).insert(TitleUI);
}

fn update(
    mut commands: Commands,
    mut clicked: EventReader<ButtonClick>,
    mut state: ResMut<State<GameState>>,
) {
    for ButtonClick(id) in clicked.iter() {
        match *id {
            TextButtonId::Title(n) => match n {
                0 => {
                    state.push(GameState::Overworld).unwrap();
                },
                1 => {
                    if let Some(text) = data::read_level() {
                        if let Some(mut decoded) = Decoder::decode_puzzle(text) {
                            decoded.prepare();
                            commands.insert_resource(CurrentPuzzle(Some(decoded)));
                            state.push(GameState::Play).unwrap();
                        }
                    }
                },
                _ => {
                    commands.insert_resource(CurrentPuzzle(Some(Puzzle::default())));
                    state.push(GameState::Editor).unwrap();
                },
            },
            _ => {}
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<TitleUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}