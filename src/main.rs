// let_(thinking) = rof if(shrink)

mod veggie;
mod loading;
mod title;
mod util;
mod grid;
mod puzzle;
mod inventory;
mod tween;

use bevy::prelude::*;
use bevy_text_mode::{TextModePlugin, TextModeTextureAtlasSprite};
use bevy_tweening::{component_animator_system, TweeningPlugin};
use crate::grid::GridPlugin;
use crate::inventory::InventoryPlugin;
use crate::loading::LoadingPlugin;
use crate::title::TitlePlugin;

// Dimensions in "zoomed" pixels (camera has a 2x factor)
pub const WIDTH: f32 = 1280. / 2.;
pub const HEIGHT: f32 = 720. / 2.;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Title,
    Puzzle,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("fff1e8").unwrap()))
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: WIDTH * 2.,
                    height: HEIGHT * 2.,
                    title: "LD52".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..Default::default()
                },
                ..default()
            })
        )
        .add_state(GameState::Loading)
        .add_plugin(TextModePlugin)
        .add_plugin(TweeningPlugin)
        .add_system(component_animator_system::<TextureAtlasSprite>)
        .add_system(component_animator_system::<TextModeTextureAtlasSprite>)
        .add_plugin(LoadingPlugin)
        .add_plugin(TitlePlugin)
        .add_plugin(GridPlugin)
        .add_plugin(InventoryPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.5, 0.5, 1.),
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., 100.),
            ..Default::default()
        },
        ..Default::default()
    });
}