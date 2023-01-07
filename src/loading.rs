use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                // .with_collection::<Fonts>()
                // .with_collection::<Sounds>()
                .with_collection::<Textures>()
                // .with_collection::<Data>()
                .continue_to_state(GameState::Title),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct Fonts {
}

#[derive(AssetCollection, Resource)]
pub struct Sounds {
}

#[derive(AssetCollection, Resource)]
pub struct Textures {
    #[asset(texture_atlas(tile_size_x = 40., tile_size_y = 40., columns = 5, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "fruit.png")]
    pub fruit: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct Data {
}