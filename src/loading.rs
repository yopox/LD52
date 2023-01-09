use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<AudioAssets>()
                .with_collection::<Textures>()
                .continue_to_state(GameState::Title),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "BGM + SFX/BGM - 1 - title.ogg")]
    pub title: Handle<AudioSource>,
    #[asset(path = "BGM + SFX/BGM - 2 - main theme.ogg")]
    pub level: Handle<AudioSource>,
    #[asset(path = "BGM + SFX/BGM - 3 - editor.ogg")]
    pub editor: Handle<AudioSource>,

    #[asset(path = "BGM + SFX/clic.ogg")]
    pub clic: Handle<AudioSource>,
    #[asset(path = "BGM + SFX/error.ogg")]
    pub error: Handle<AudioSource>,
    #[asset(path = "BGM + SFX/place.ogg")]
    pub place: Handle<AudioSource>,
    #[asset(path = "BGM + SFX/sfx.ogg")]
    pub win: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 40., tile_size_y = 40., columns = 7, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "veggies.png")]
    pub fruit: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 4, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "faces.png")]
    pub faces: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 5, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "border.png")]
    pub border: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 40., tile_size_y = 40., columns = 4, rows = 1, padding_x = 2., padding_y = 0.))]
    #[asset(path = "tile.png")]
    pub tile: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 32, rows = 32, padding_x = 0., padding_y = 0.))]
    #[asset(path = "MRMOTEXT EX.png")]
    pub mrmotext: Handle<TextureAtlas>,

    #[asset(path = "title.png")]
    pub title: Handle<Image>,

    #[asset(path = "heart.png")]
    pub heart: Handle<Image>,
}