// let_(thinking) = rof if(shrink)

use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_text_mode::{TextModePlugin, TextModeTextureAtlasSprite};
use bevy_tweening::{component_animator_system, TweeningPlugin};

use crate::audio::InternalAudioPlugin;
use crate::editor::EditorPlugin;
use crate::grid::GridPlugin;
use crate::inventory::InventoryPlugin;
use crate::loading::LoadingPlugin;
use crate::overworld::OverworldPlugin;
use crate::play::PlayPlugin;
use crate::text::TextPlugin;
use crate::title::TitlePlugin;
use crate::tutorial::TutorialPlugin;
use crate::util::Colors;
use crate::veggie::VeggiePlugin;

mod veggie;
mod loading;
mod title;
mod util;
mod grid;
mod puzzle;
mod inventory;
mod tween;
mod text;
mod editor;
mod data;
mod levels;
mod play;
mod tutorial;
mod progress;
mod overworld;
mod audio;

// Dimensions in "zoomed" pixels (camera has a 2x factor)
pub const WIDTH: f32 = 1280. / 2.;
pub const HEIGHT: f32 = 720. / 2.;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum GameState {
    Loading,
    Title,
    Play,
    Editor,
    Tutorial,
    Overworld,
}

#[derive(Resource)]
pub struct BlockInput(pub bool);

fn main() {
    App::new()
        .insert_resource(ClearColor(Colors::DarkRed.get()))
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
        .add_plugin(InternalAudioPlugin)
        .add_plugin(VeggiePlugin)
        .add_plugin(TextPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(TitlePlugin)
        .add_plugin(OverworldPlugin)
        .add_plugin(PlayPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(TutorialPlugin)
        .add_startup_system(init)
        .insert_resource(BlockInput(false))
        .insert_resource(PkvStore::new("yopox.ld52", "mad_veggies"))
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