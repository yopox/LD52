use std::ops::Add;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use bevy_tweening::Animator;
use strum::IntoEnumIterator;

use crate::{data, GameState, HEIGHT, puzzle, util, WIDTH};
use crate::audio::{BGM, PlayBgmEvent, PlaySfxEvent, SFX};
use crate::data::{Decoder, Encoder};
use crate::grid::{CurrentPuzzle, DisplayLevel, GridChanged, GridTile, PreviousPos};
use crate::loading::Textures;
use crate::puzzle::{Puzzle, Tile};
use crate::text::{ButtonClick, spawn_text, TextButtonId};
use crate::util::{Colors, text_mode_bundle};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Editor)
                .with_system(play_music)
            )
            .add_system_set(SystemSet::on_update(GameState::Editor)
                .with_system(display_editor)
                .with_system(handle_click)
                .with_system(handle_drop)
                .with_system(handle_click_on_grid)
                .with_system(update_author)
                .with_system(click_on_button.after("logic"))
            )
            .add_system_set(SystemSet::on_exit(GameState::Editor).with_system(cleanup));
    }
}

#[derive(Component)]
struct EditorUI;

#[derive(Component)]
struct EditorTile(Tile);

#[derive(Component)]
struct AuthorName;

fn play_music(
   mut bgm: EventWriter<PlayBgmEvent>,
) {
    bgm.send(PlayBgmEvent(BGM::Editor));
}

fn display_editor(
    mut commands: Commands,
    textures: Res<Textures>,
    puzzle: Res<CurrentPuzzle>,
    mut display_event: EventReader<DisplayLevel>,
    entities: Query<Entity, With<EditorUI>>,
) {
    if puzzle.0.is_none() { return; }
    let puzzle = puzzle.0.as_ref().unwrap();

    for _ in display_event.iter() {
        entities.iter().for_each(|e| commands.entity(e).despawn_recursive());

        // Tiles
        let tiles = Tile::iter().collect::<Vec<Tile>>();
        let h = (HEIGHT - tiles.len() as f32 * 48.) / 2. + 48.;
        let w = WIDTH - 32. - 48.;

        let id = util::frame(
            &mut commands, &textures,
            w - 16., h - 8., util::z::VEG_UI_BG,
            9, 14,
            Colors::DarkRed, Colors::Beige
        );
        commands.entity(id).insert(EditorUI);

        for (i, tile) in Tile::iter().enumerate() {
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: tile.index(),
                        anchor: Anchor::BottomLeft,
                        ..Default::default()
                    },
                    texture_atlas: textures.tile.clone(),
                    transform: Transform::from_xyz(w, h + 48. * i as f32 + 4., util::z::VEG_UI),
                    ..Default::default()
                })
                .insert(EditorUI)
                .insert(EditorTile(tile.clone()));
        }

        // Buttons
        let grid_w = puzzle.size.0 as f32 * 40.;
        let grid_x = (WIDTH - grid_w) / 2.;
        let grid_h = puzzle.size.1 as f32 * 40.;
        let grid_y = (HEIGHT - grid_h) / 2.;

        #[cfg(target_arch = "wasm32")]
            let save =      "- save  -\n  level  ";
        #[cfg(not(target_arch = "wasm32"))]
            let save = "save to  \nclipboard";

        #[cfg(target_arch = "wasm32")]
            let load =      "- load  -\n  level";
        #[cfg(not(target_arch = "wasm32"))]
            let load = "load from\nclipboard";

        for (x, y, text, bg, fg, button) in [
            (grid_x, grid_y + grid_h + 8., "+", Colors::Green, Colors::Beige, TextButtonId::ExpandShrink(true, true)),
            (grid_x + 12., grid_y + grid_h + 8., "-", Colors::Red, Colors::Beige, TextButtonId::ExpandShrink(false, true)),
            (grid_x + grid_w + 8., grid_y + grid_h - 8., "+", Colors::Green, Colors::Beige, TextButtonId::ExpandShrink(true, false)),
            (grid_x + grid_w + 8., grid_y + grid_h - 20., "-", Colors::Red, Colors::Beige, TextButtonId::ExpandShrink(false, false)),
            (WIDTH - 96., 62. + 16., save, Colors::Beige, Colors::DarkRed, TextButtonId::Export),
            (WIDTH - 96., 62. - 8., load, Colors::Beige, Colors::DarkRed, TextButtonId::Import),
            (WIDTH - 96., 62. - 32., "- clear -", Colors::Beige, Colors::DarkRed, TextButtonId::Clear),
            (WIDTH - 96., 62. - 48., "- leave -", Colors::Beige, Colors::DarkRed, TextButtonId::LeaveEditor),
        ] {
            let id = spawn_text(
                &mut commands,
                &textures,
                Vec3::new(x, y, util::z::VEG_UI),
                text,
                bg,
                fg,
            );

            commands
                .entity(id)
                .insert(button)
                .insert(EditorUI);
        }

        let id = spawn_text(
            &mut commands,
            &textures,
            Vec3::new(WIDTH - 96., 62. + 40., util::z::VEG_UI),
            "author:",
            Colors::Beige,
            Colors::DarkRed,
        );
        commands.entity(id).insert(EditorUI);

        let id = spawn_text(
            &mut commands,
            &textures,
            Vec3::new(WIDTH - 96., 62. + 32., util::z::VEG_UI),
            if puzzle.author.is_empty() { "type name" } else { &puzzle.author },
            Colors::Beige,
            Colors::DarkRed,
        );
        commands
            .entity(id)
            .insert(EditorUI)
            .insert(AuthorName);

        for i in 0..11 {
            commands
                .spawn(text_mode_bundle(
                    &Colors::DarkRed,
                    &Colors::Beige,
                    9 * 32 + 25,
                    WIDTH - 104. + 8. * i as f32, 62. + 56., util::z::VEG_UI,
                    textures.mrmotext.clone(),
                ))
                .insert(EditorUI);
        }
        commands
            .spawn(TextModeSpriteSheetBundle {
                sprite: TextModeTextureAtlasSprite {
                    fg: Colors::Beige.get(),
                    bg: Colors::Beige.get(),
                    index: 0,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: textures.mrmotext.clone(),
                transform: Transform {
                    translation: Vec3::new(WIDTH - 104., 0., 0.),
                    scale: Vec3::new(11., 15., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(EditorUI);

        break;
    }
    display_event.clear();
}

#[derive(Component)]
pub struct DraggedTile(pub Tile);

fn handle_click(
    mut commands: Commands,
    inventory: Query<(&EditorTile, &Transform)>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    textures: Res<Textures>,
    mut sfx: EventWriter<PlaySfxEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            if let Some((e, _)) = inventory.iter().filter(|(_, t)|
                (t.translation.x + 20. - pos.x / 2.).abs() < 20.
                    && (t.translation.y + 20. - pos.y / 2.).abs() < 20.
            ).nth(0) {
                sfx.send(PlaySfxEvent(SFX::Clic));

                // Spawn a tile
                let tile_e = commands
                    .spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: e.0.index(),
                            anchor: Anchor::BottomLeft,
                            ..Default::default()
                        },
                        texture_atlas: textures.tile.clone(),
                        transform: Transform::from_xyz(pos.x / 2. - 20., pos.y / 2. - 20. + 8., util::z::VEG_DRAG),
                        ..Default::default()
                    })
                    .id();

                commands
                    .entity(tile_e)
                    .insert(EditorUI)
                    .insert(DraggedTile(e.0.clone()));
            }
        }
    }
}

fn handle_drop(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    query: Query<(Entity, &DraggedTile, &Transform, Option<&PreviousPos>)>,
    mut puzzle: ResMut<CurrentPuzzle>,
    mut grid_changed: EventWriter<GridChanged>,
    mut sfx: EventWriter<PlaySfxEvent>,
) {
    if puzzle.0.is_none() { return; }
    let puzzle = puzzle.0.as_mut().unwrap();

    let animation_len = 1000;

    if mouse.just_released(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            for (e, dragged, t, prev) in query.iter() {
                commands.entity(e).remove::<DraggedTile>();

                // Drop on a free tile of the grid -> animate to pos + update count
                if let Some(tile) = crate::grid::get_pos_at(pos, puzzle.size) {
                    let destination = if !puzzle.placed.contains_key(&tile) && !puzzle.tiles.contains_key(&tile) {
                        Some(tile)
                    } else if prev.is_some() {
                        Some(prev.unwrap().0)
                    } else {
                        None
                    };
                    if destination.is_some() {
                        sfx.send(PlaySfxEvent(SFX::Place));

                        let tile = destination.unwrap();
                        puzzle.tiles.insert(tile, dragged.0.clone());

                        commands
                            .entity(e)
                            .remove::<PreviousPos>()
                            .insert(Animator::<Transform>::new(
                                crate::tween::position_out(
                                    t.translation.xy(),
                                    crate::grid::get_tile_pos(tile, puzzle.size),
                                    util::z::VEGGIE,
                                    animation_len / 2
                                )
                            ))
                            .insert(GridTile(dragged.0.clone(), tile));

                        grid_changed.send(GridChanged);

                        continue
                    }
                }

                // Else -> disappear animation
                commands
                    .entity(e)
                    .insert(Animator::<Transform>::new(
                        crate::tween::position_out(
                            t.translation.xy(),
                            t.translation.xy().add(Vec2::new(0., 24.)),
                            t.translation.z,
                            3 * animation_len
                        )
                    ))
                    .insert(Animator::<TextureAtlasSprite>::new(
                        crate::tween::tween_texture_atlas_sprite_opacity(animation_len, false)
                    ));
            }
        }
    }
}

fn handle_click_on_grid(
    mut commands: Commands,
    mut tiles: Query<(Entity, &GridTile, &mut Transform)>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut puzzle: ResMut<CurrentPuzzle>,
    mut grid_changed: EventWriter<GridChanged>,
) {
    if puzzle.0.is_none() { return; }
    let puzzle = puzzle.0.as_mut().unwrap();

    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            if let Some((e, grid_tile, mut t)) = tiles.iter_mut().filter(|(_, _, t)|
                (t.translation.x + 20. - pos.x / 2.).abs() < 20.
                    && (t.translation.y + 20. - pos.y / 2.).abs() < 20.
            ).nth(0) {
                commands
                    .entity(e)
                    .insert(DraggedTile(grid_tile.0.clone()))
                    .insert(PreviousPos(grid_tile.1))
                    .remove::<GridTile>();
                puzzle.tiles.remove(&grid_tile.1);
                t.translation.z = util::z::VEG_DRAG;
                grid_changed.send(GridChanged);
            }
        }
    }
}

fn click_on_button(
    mut commands: Commands,
    mut clicks: EventReader<ButtonClick>,
    mut current_puzzle: ResMut<CurrentPuzzle>,
    mut display_level: EventWriter<DisplayLevel>,
    mut state: ResMut<State<GameState>>,
    mut sfx: EventWriter<PlaySfxEvent>,
) {
    if current_puzzle.0.is_none() { return; }
    let mut puzzle = current_puzzle.0.as_mut().unwrap();

    for click in clicks.iter() {
        match click.0 {
            TextButtonId::ExpandShrink(expand, rows) => {
                // Expand at max size or shrink at min size -> impossible
                if expand && (rows && puzzle.size.1 >= puzzle::MAX_H || !rows && puzzle.size.0 >= puzzle::MAX_W)
                    || !expand && (rows && puzzle.size.1 == 1 || !rows && puzzle.size.0 == 1) { return; }

                // Update puzzle dimensions
                if expand {
                    if rows { puzzle.size.1 += 1; } else { puzzle.size.0 += 1; }
                } else {
                    if rows {
                        puzzle.size.1 -= 1;
                        // Remove placed veggies / tiles on the removed row
                        (0..puzzle.size.0).for_each(|x| {
                            puzzle.tiles.remove(&(x, puzzle.size.1));
                            puzzle.placed.remove(&(x, puzzle.size.1));
                        })
                    } else {
                        puzzle.size.0 -= 1;
                        // Remove placed veggies / tiles on the removed line
                        (0..puzzle.size.1).for_each(|y| {
                            puzzle.tiles.remove(&(puzzle.size.0, y));
                            puzzle.placed.remove(&(puzzle.size.0, y));
                        })
                    }
                }

                // Reposition stuff
                display_level.send(DisplayLevel);
            }
            TextButtonId::Export => {
                if let Some(text) = Encoder::encode_puzzle(&puzzle) {
                    data::write_level(text);
                } else {
                    sfx.send(PlaySfxEvent(SFX::Error));
                }
            }

            TextButtonId::Import => {
                if let Some(text) = data::read_level() {
                    if let Some(decoded) = Decoder::decode_puzzle(text) {
                        commands.insert_resource(CurrentPuzzle(Some(decoded)));
                        display_level.send(DisplayLevel);
                    }
                }
            }

            TextButtonId::Clear => {
                commands.insert_resource(CurrentPuzzle(Some(Puzzle::default())));
                display_level.send(DisplayLevel);
            }

            TextButtonId::LeaveEditor => {
                state.pop().unwrap_or_default();
            }

            _ => {}
        }
    }
}

fn update_author(
    keyboard_input: Res<Input<KeyCode>>,
    mut puzzle: ResMut<CurrentPuzzle>,
    mut refresh: EventWriter<DisplayLevel>,
) {
    for code in keyboard_input.get_just_pressed() {
        match get_char(code) {
            Some('<') => { puzzle.0.as_mut().unwrap().author.pop(); },
            Some(c) => {
                let p = puzzle.0.as_mut().unwrap();
                if p.author.len() < 9 { p.author.push(c); }
            },
            None => { return; },
        }
        refresh.send(DisplayLevel);
    }
}

fn get_char(code: &KeyCode) -> Option<char> {
    match code {
        KeyCode::A => Some('a'),
        KeyCode::B => Some('b'),
        KeyCode::C => Some('c'),
        KeyCode::D => Some('d'),
        KeyCode::E => Some('e'),
        KeyCode::F => Some('f'),
        KeyCode::G => Some('g'),
        KeyCode::H => Some('h'),
        KeyCode::I => Some('i'),
        KeyCode::J => Some('j'),
        KeyCode::K => Some('k'),
        KeyCode::L => Some('l'),
        KeyCode::M => Some('m'),
        KeyCode::N => Some('n'),
        KeyCode::O => Some('o'),
        KeyCode::P => Some('p'),
        KeyCode::Q => Some('q'),
        KeyCode::R => Some('r'),
        KeyCode::S => Some('s'),
        KeyCode::T => Some('t'),
        KeyCode::U => Some('u'),
        KeyCode::V => Some('v'),
        KeyCode::W => Some('w'),
        KeyCode::X => Some('x'),
        KeyCode::Y => Some('y'),
        KeyCode::Z => Some('z'),
        KeyCode::Back => Some('<'),
        _ => None,
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<EditorUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}