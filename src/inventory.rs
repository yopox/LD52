use std::ops::Add;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;
use bevy_tweening::Animator;
use strum::IntoEnumIterator;

use crate::{BlockInput, GameState, HEIGHT, util};
use crate::editor::DraggedTile;
use crate::grid::{CurrentPuzzle, DisplayLevel, GridChanged, GridUI, GridVeggie, PreviousPos};
use crate::loading::Textures;
use crate::text::{ChangeText, spawn_text};
use crate::util::Colors;
use crate::veggie::{Expression, spawn_veggie, UpdateFaces, Veggie};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        for state in [GameState::Play, GameState::Editor] {
            app
                .add_system_set(SystemSet::on_update(state)
                    .with_system(display)
                    .with_system(update_counts)
                    .with_system(handle_click.label("logic"))
                    .with_system(update_dragged.label("logic"))
                    .with_system(handle_drop.label("logic"))
                )
                .add_system_set(SystemSet::on_exit(state).with_system(cleanup));
        }
    }
}

#[derive(Component)]
struct InventoryUI;

#[derive(Component)]
struct InventoryVeg(Veggie);

#[derive(Component)]
struct InventoryCount(Veggie);

fn display(
    mut commands: Commands,
    mut event: EventReader<DisplayLevel>,
    puzzle: Res<CurrentPuzzle>,
    textures: Res<Textures>,
    state: Res<State<GameState>>,
) {
    let in_editor = state.current() == &GameState::Editor;

    for _ in event.iter() {
        if let Some(puzzle) = &puzzle.0 {
            let all_veggies = if in_editor {
                Veggie::iter().map(|v| (v, 99)).collect::<Vec<(Veggie, u8)>>()
            } else {
                puzzle.veggies.iter().map(|(v, c)| (*v, *c)).collect::<Vec<(Veggie, u8)>>()
            };

            let h = (HEIGHT - all_veggies.len() as f32 * 48.) / 2.;
            let w = 24.;
            for (i, (veg, count)) in all_veggies.iter().enumerate() {
                let veg_e = spawn_veggie(
                    &mut commands,
                    &textures,
                    Vec3::new(w + if in_editor { 16. } else { 0. }, h + 48. * i as f32 + 4., util::z::VEG_UI),
                    veg,
                    Expression::Neutral,
                );
                commands
                    .entity(veg_e)
                    .insert(InventoryUI)
                    .insert(InventoryVeg(veg.clone()));

                if !in_editor {
                    let text = spawn_text(
                        &mut commands,
                        &textures,
                        Vec3::new(w + 40., h + 48. * i as f32 + 8., util::z::COUNT_TEXT),
                        &format!("x{:0>2}", count),
                        Colors::Beige.get(),
                        Colors::DarkRed.get(),
                    );

                    commands
                        .entity(text)
                        .insert(InventoryUI)
                        .insert(InventoryCount(veg.clone()));
                }
            }

            // Veggies frame
            let id = util::frame(
                &mut commands, &textures,
                w - 8., h, util::z::VEG_UI_BG,
                11, all_veggies.len() * 6,
                Colors::DarkRed, Colors::Beige
            );

            commands.entity(id).insert(InventoryUI);
        }

        // qVSz1w5Yl_+k*B
    }
}

fn update_counts(
    counts: Query<(&InventoryCount, Entity)>,
    puzzle: Res<CurrentPuzzle>,
    mut grid_changed: EventReader<GridChanged>,
    mut change_text: EventWriter<ChangeText>,
    state: Res<State<GameState>>,
) {
    let in_editor = state.current() == &GameState::Editor;
    if puzzle.0.is_none() { return; }
    let puzzle = puzzle.0.as_ref().unwrap();

    for _ in grid_changed.iter() {
        for (inv_count, e) in counts.iter() {
            let available = puzzle.remaining_veggie(&inv_count.0, in_editor);
            let text = format!("x{:0>2}", available);
            change_text.send(ChangeText(e, text));
        }
        break;
    }

    grid_changed.clear();
}

#[derive(Component)]
pub struct DraggedVeg(pub Veggie);

fn handle_click(
    mut commands: Commands,
    inventory: Query<(&InventoryVeg, &Transform)>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    textures: Res<Textures>,
    puzzle: Res<CurrentPuzzle>,
    state: Res<State<GameState>>,
    block_input: Res<BlockInput>,
) {
    let in_editor = state.current() == &GameState::Editor;
    if puzzle.0.is_none() || block_input.0 { return; }
    let puzzle = puzzle.0.as_ref().unwrap();

    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            if let Some((v, _)) = inventory.iter().filter(|(_, t)|
                (t.translation.x + 20. - pos.x / 2.).abs() < 20.
                && (t.translation.y + 20. - pos.y / 2.).abs() < 20.
            ).nth(0) {
                if puzzle.remaining_veggie(&v.0, in_editor) == 0 { return; }

                // Spawn a veggie
                let veg_e = spawn_veggie(
                    &mut commands,
                    &textures,
                    Vec3::new(pos.x / 2. - 20., pos.y / 2. - 20. + 8., util::z::VEG_DRAG),
                    &v.0,
                    Expression::Surprised,
                );
                commands
                    .entity(veg_e)
                    .insert(InventoryUI)
                    .insert(DraggedVeg(v.0.clone()));
            }
        }
    }
}

fn update_dragged(
    mut query: Query<&mut Transform, Or<(With<DraggedVeg>, With<DraggedTile>)>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        query.iter_mut().for_each(|mut t| {
            t.translation.x = pos.x / 2. - 20.;
            t.translation.y = pos.y / 2. - 20. + 8.;
        })
    }
}

fn handle_drop(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    query: Query<(Entity, &DraggedVeg, &Transform, &Children, Option<&PreviousPos>)>,
    mut puzzle: ResMut<CurrentPuzzle>,
    mut update_faces: EventWriter<UpdateFaces>,
    mut grid_changed: EventWriter<GridChanged>,
) {
    if puzzle.0.is_none() { return; }
    let puzzle = puzzle.0.as_mut().unwrap();

    let animation_len = 1000;

    if mouse.just_released(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            for (e, v, t, c, prev) in query.iter() {
                commands.entity(e)
                    .remove::<DraggedVeg>()
                    .insert(GridUI);

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
                        let tile = destination.unwrap();
                        puzzle.placed.insert(tile, v.0.clone());

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
                            .insert(GridVeggie(v.0.clone(), tile, (false, false)));

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

                for face in c {
                    commands
                        .entity(*face)
                        .insert(Animator::<TextModeTextureAtlasSprite>::new(
                            crate::tween::tween_text_mode_sprite_opacity(animation_len, false)
                        ));
                }
                update_faces.send(UpdateFaces(e, (Expression::Sad, Expression::Sad)));
            }
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<InventoryUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}