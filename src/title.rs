use bevy::prelude::*;
use crate::{GameState, util, WIDTH};
use crate::loading::Textures;
use crate::veggie::{spawn_veggie, Veggie};

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

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    let w = (WIDTH / 2. - 40. * 7.) / 2.;
    let h = 196.;
    spawn_veggie(
        &mut commands,
        &textures,
        Vec3::new(w, h, util::z::VEGGIE),
        &Veggie::Strawberry,
    );
    spawn_veggie(
        &mut commands,
        &textures,
        Vec3::new(w + 40., h, util::z::VEGGIE),
        &Veggie::Apple,
    );
    spawn_veggie(
        &mut commands,
        &textures,
        Vec3::new(w + 2. * 40., h, util::z::VEGGIE),
        &Veggie::Tomato,
    );
    spawn_veggie(
        &mut commands,
        &textures,
        Vec3::new(w + 3. * 40., h, util::z::VEGGIE),
        &Veggie::Carrot,
    );
    spawn_veggie(
        &mut commands,
        &textures,
        Vec3::new(w + 4. * 40., h, util::z::VEGGIE),
        &Veggie::Cherry,
    );
    spawn_veggie(
        &mut commands,
        &textures,
        Vec3::new(w + 5. * 40., h, util::z::VEGGIE),
        &Veggie::Garlic,
    );
    spawn_veggie(
        &mut commands,
        &textures,
        Vec3::new(w + 6. * 40., h, util::z::VEGGIE),
        &Veggie::Mint,
    );
}

fn update() {}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<TitleUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}