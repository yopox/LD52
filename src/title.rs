use arboard::Clipboard;
use bevy::prelude::*;
use rand::random;
use strum::IntoEnumIterator;
use crate::{GameState, HEIGHT, util, WIDTH};
use crate::data::Decoder;
use crate::editor::InEditor;
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
            .add_system_set(SystemSet::on_update(GameState::Title).with_system(update))
            .add_system_set(SystemSet::on_exit(GameState::Title).with_system(cleanup));
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
) {
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

    // Buttons
    for (text, x, y, button) in [
        ("---level-----\n------list---".to_string(), WIDTH / 2. - 8. * 5.5, 128. + 16., TextButtonId::Title(0)),
        ("--load from--\n--clipboard--".to_string(), WIDTH / 2. - 8. * 5.5, 128. - 16., TextButtonId::Title(1)),
        ("---level-----\n----editor---".to_string(), WIDTH / 2. - 8. * 5.5, 128. - 48., TextButtonId::Title(2)),
    ] {
        let id = spawn_text(
            &mut commands,
            &textures,
            Vec3::new(x, y, util::z::VEG_UI),
            &text,
            Colors::DarkRed.get(),
            Colors::Beige.get(),
        );

        commands
            .entity(id)
            .insert(button)
            .insert(TitleUI);
    }
}

fn update(
    mut commands: Commands,
    mut clicked: EventReader<ButtonClick>,
    mut state: ResMut<State<GameState>>,
    mut in_editor: ResMut<InEditor>,
) {
    for ButtonClick(id) in clicked.iter() {
        match *id {
            TextButtonId::Title(n) => match n {
                0 => {
                    in_editor.0 = false;
                    state.set(GameState::Puzzle).unwrap_or_default();
                },
                1 => {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        if let Ok(text) = clipboard.get_text() {
                            if let Some(mut decoded) = Decoder::decode_puzzle(text) {
                                decoded.placed.clear();
                                commands.insert_resource(CurrentPuzzle(Some(decoded)));
                                in_editor.0 = false;
                                state.set(GameState::Puzzle).unwrap_or_default();
                            }
                        }
                    }
                },
                _ => {
                    in_editor.0 = true;
                    commands.insert_resource(CurrentPuzzle(Some(Puzzle::default())));
                    state.set(GameState::Puzzle).unwrap_or_default();
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