use bevy::prelude::*;
use strum::IntoEnumIterator;
use crate::{GameState, HEIGHT, util};
use crate::grid::{CurrentPuzzle, DisplayLevel};
use crate::loading::Textures;
use crate::veggie::{Expression, spawn_veggie, Veggie};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Puzzle)
                .with_system(display)
            )
            .add_system_set(SystemSet::on_exit(GameState::Puzzle).with_system(cleanup));
    }
}

#[derive(Component)]
struct InventoryUI;

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
                commands.entity(veg_e).insert(InventoryUI);
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