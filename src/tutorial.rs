use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{GameState, progress};

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Tutorial).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Tutorial).with_system(update))
            .add_system_set(SystemSet::on_exit(GameState::Tutorial).with_system(cleanup));
    }
}

#[derive(Component)]
struct TutorialUI;

fn setup(
    mut commands: Commands,
    pkv: Res<PkvStore>,
) {
    let progress = progress::get_progress(&pkv);
}

fn update() {}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<TutorialUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}