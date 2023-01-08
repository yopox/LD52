use std::ops::Add;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;
use bevy_tweening::Animator;
use strum::IntoEnumIterator;
use crate::{GameState, HEIGHT, util};
use crate::grid::{CurrentPuzzle, DisplayLevel, GridChanged, GridVeggie};
use crate::loading::Textures;
use crate::veggie::{Expression, spawn_veggie, UpdateFaces, Veggie};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Puzzle)
                .with_system(display)
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
            }
        }
    }
}

#[derive(Component)]
pub struct DraggedVeg(pub Veggie);

fn handle_click(
    mut commands: Commands,
    inventory: Query<(&InventoryVeg, &Transform)>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    textures: Res<Textures>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            if let Some((v, t)) = inventory.iter().filter(|(_, t)|
                (t.translation.x + 20. - pos.x / 2.).abs() < 20.
                && (t.translation.y + 20. - pos.y / 2.).abs() < 20.
            ).nth(0) {
                // Spawn a veggie
                let veg_e = spawn_veggie(
                    &mut commands,
                    &textures,
                    Vec3::new(pos.x / 2. - 20., pos.y / 2. - 20., util::z::VEG_DRAG),
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
            t.translation.y = pos.y / 2. - 20.;
        })
    }
}

fn handle_drop(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    query: Query<(Entity, &DraggedVeg, &Transform, &Children)>,
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
            for (e, v, t, c) in query.iter() {
                commands.entity(e).remove::<DraggedVeg>();

                // Drop on a free tile of the grid -> animate to pos + update count
                if let Some(tile) = crate::grid::get_pos_at(pos, puzzle.size) {
                    if !puzzle.placed.contains_key(&tile) && !puzzle.tiles.contains_key(&tile) {
                        puzzle.placed.insert(tile, v.0.clone());

                        commands
                            .entity(e)
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
                info!("inventory::handle_drop UpdateFaces {:?};{:?}", Expression::Sad, Expression::Sad);}
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