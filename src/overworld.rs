use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::GameState;
use crate::loading::Textures;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(setup))
            .add_system_set(SystemSet::on_resume(GameState::Overworld).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Overworld).with_system(update))
            .add_system_set(SystemSet::on_exit(GameState::Overworld).with_system(cleanup))
            .add_system_set(SystemSet::on_pause(GameState::Overworld).with_system(cleanup))
        ;
    }
}

#[derive(Component)]
struct OverworldUI;

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    pkv: Res<PkvStore>,
) {

}

fn update() {}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<OverworldUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}