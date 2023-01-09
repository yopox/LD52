use bevy::prelude::{Res, ResMut};
use bevy::utils::HashSet;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Progress {
    finished_levels: HashSet<usize>,
    tutorial: HashSet<usize>,
    custom_levels: Vec<String>,
}

const KEY: &'static str = "progress";

pub fn get_progress(
    pkv: &Res<PkvStore>
) -> Progress {
    pkv.get::<Progress>(KEY).unwrap_or_default()
}

pub fn set_progress(
    mut pkv: &mut ResMut<PkvStore>,
    progress: &Progress,
) {
    pkv.set::<Progress>(KEY, progress).unwrap_or_default();
}