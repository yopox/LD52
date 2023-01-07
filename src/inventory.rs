use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;
use strum::IntoEnumIterator;
use crate::{GameState, HEIGHT, util};
use crate::grid::{CurrentPuzzle, DisplayLevel};
use crate::loading::Textures;
use crate::veggie::{Expression, Face, spawn_veggie, Veggie};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Puzzle)
                .with_system(display)
                .with_system(handle_click)
                .with_system(update_dragged)
                .with_system(handle_drop)
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
    mut query: Query<(Entity, &DraggedVeg, &mut Transform, &Children)>,
    mut faces: Query<&mut TextModeTextureAtlasSprite, With<Face>>,
    mut puzzle: ResMut<CurrentPuzzle>,
) {
    if puzzle.0.is_none() { return; }
    let mut puzzle = puzzle.0.as_mut().unwrap();

    if mouse.just_released(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            for (e, v, mut t, c) in query.iter_mut() {
                commands.entity(e).remove::<DraggedVeg>();

                // Drop on a free tile of the grid -> animate to pos + update count
                if let Some(tile) = crate::grid::get_pos_at(pos, puzzle.size) {
                    if !puzzle.placed.contains_key(&tile) && !puzzle.tiles.contains_key(&tile) {
                        puzzle.placed.insert(tile, v.0.clone());
                        continue
                    }
                }

                // Else -> disappear animation
                for face in c {
                    let mut sprite = faces.get_mut(*face).unwrap();
                    sprite.index = Expression::Sad.index();
                }
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