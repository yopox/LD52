use bevy::utils::HashSet;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Progress {
    pub finished_levels: HashSet<usize>,
    pub tutorial: HashSet<u8>,
    pub custom_levels: Vec<(String, bool)>,
    pub finished_custom: HashSet<usize>,
}

const KEY: &'static str = "progress";

pub fn get_progress(
    pkv: &PkvStore
) -> Progress {
    pkv.get::<Progress>(KEY).unwrap_or_default()
}

pub fn set_progress(
    mut pkv: &mut PkvStore,
    progress: &Progress,
) {
    pkv.set::<Progress>(KEY, progress).unwrap_or_default();
}