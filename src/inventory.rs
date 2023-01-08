use std::ops::Add;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;
use bevy_tweening::Animator;
use strum::IntoEnumIterator;
use crate::{GameState, HEIGHT, util};
use crate::grid::{CurrentPuzzle, DisplayLevel, GridChanged, GridVeggie, PreviousPos};
use crate::loading::Textures;
use crate::puzzle::Puzzle;
use crate::text::{ChangeText, spawn_text};
use crate::util::Colors;
use crate::veggie::{Expression, spawn_veggie, UpdateFaces, Veggie};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Puzzle)
                .with_system(display)
                .with_system(update_counts)
                .with_system(handle_click.label("logic"))
                .with_system(update_dragged.label("logic"))
                .with_system(handle_drop.label("logic"))
            )
            .add_system_set(SystemSet::on_exit(GameState::Puzzle).with_system(cleanup));
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
) {
    for _ in event.iter() {
        if let Some(puzzle) = &puzzle.0 {
            let h = (HEIGHT - puzzle.veggies.len() as f32 * 48.) / 2.;
            let w = 32.;

            for (i, (veg, count)) in puzzle.veggies.iter().enumerate() {
                let veg_e = spawn_veggie(
                    &mut commands,
                    &textures,
                    Vec3::new(w, h + 48. * i as f32 + 4., util::z::VEG_UI),
                    veg,
                    Expression::Neutral,
                );
                commands
                    .entity(veg_e)
                    .insert(InventoryUI)
                    .insert(InventoryVeg(veg.clone()));

                let text = spawn_text(
                    &mut commands,
                    &textures,
                    Vec3::new(w + 40., h + 48. * i as f32 + 8., util::z::COUNT_TEXT),
                    &format!("x{:0>2}", count),
                    Colors::DarkRed.get(),
                    Colors::Beige.get(),
                );

                commands
                    .entity(text)
                    .insert(InventoryUI)
                    .insert(InventoryCount(veg.clone()));
            }
        }
    }
}

fn update_counts(
    counts: Query<(&InventoryCount, Entity)>,
    puzzle: Res<CurrentPuzzle>,
    mut grid_changed: EventReader<GridChanged>,
    mut change_text: EventWriter<ChangeText>,
) {
    if puzzle.0.is_none() { return; }
    let puzzle = puzzle.0.as_ref().unwrap();

    for _ in grid_changed.iter() {
        for (inv_count, e) in counts.iter() {
            let available = puzzle.remaining_veggie(&inv_count.0);
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
) {
    if puzzle.0.is_none() { return; }
    let puzzle = puzzle.0.as_ref().unwrap();

    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            if let Some((v, t)) = inventory.iter().filter(|(_, t)|
                (t.translation.x + 20. - pos.x / 2.).abs() < 20.
                && (t.translation.y + 20. - pos.y / 2.).abs() < 20.
            ).nth(0) {
                if puzzle.remaining_veggie(&v.0) == 0 { return; }

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
    mut query: Query<&mut Transform, With<DraggedVeg>>,
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
    let mut puzzle = puzzle.0.as_mut().unwrap();

    let animation_len = 1000;

    if mouse.just_released(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            for (e, v, t, c, prev) in query.iter() {
                commands.entity(e).remove::<DraggedVeg>();

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
                                    crate::grid::get_tile_pos(tile, puzzle.size)
                                        .add(Vec2::new(0., 0.)),
                                    util::z::VEGGIE,
                                    animation_len / 2
                                )
                            ))
                            .insert(GridVeggie(v.0.clone(), tile, (false, false)));

                        grid_changed.send(GridChanged);

                        continue
                    }
                }

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

                // Else -> disappear animation
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