use std::ops::Add;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_tweening::Animator;
use strum::IntoEnumIterator;
use crate::{GameState, HEIGHT, util, WIDTH};
use crate::grid::{CurrentPuzzle, GridChanged, GridTile, PreviousPos};
use crate::loading::Textures;
use crate::puzzle::Tile;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InEditor(true))
            .add_system_set(SystemSet::on_enter(GameState::Puzzle).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Puzzle)
                .with_system(handle_click)
                .with_system(handle_drop)
                .with_system(handle_click_on_grid)
            )
            .add_system_set(SystemSet::on_exit(GameState::Puzzle).with_system(cleanup));
    }
}

#[derive(Resource)]
pub struct InEditor(pub bool);

#[derive(Component)]
struct EditorUI;

#[derive(Component)]
struct EditorTile(Tile);

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    in_editor: Res<InEditor>,
) {
    if !in_editor.0 { return; }

    let tiles = Tile::iter().collect::<Vec<Tile>>();

    let h = (HEIGHT - tiles.len() as f32 * 48.) / 2.;
    let w = WIDTH - 32. - 40.;

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
}

#[derive(Component)]
pub struct DraggedTile(pub Tile);

fn handle_click(
    mut commands: Commands,
    inventory: Query<(&EditorTile, &Transform)>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    textures: Res<Textures>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            if let Some((e, t)) = inventory.iter().filter(|(_, t)|
                (t.translation.x + 20. - pos.x / 2.).abs() < 20.
                    && (t.translation.y + 20. - pos.y / 2.).abs() < 20.
            ).nth(0) {

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
) {
    if puzzle.0.is_none() { return; }
    let mut puzzle = puzzle.0.as_mut().unwrap();

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

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<EditorUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
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
    let mut puzzle = puzzle.0.as_mut().unwrap();

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