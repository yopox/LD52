use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_kira_audio::prelude::*;

use crate::loading::AudioAssets;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AudioPlugin)
            .add_system(update_bgm)
            .add_event::<PlayBgmEvent>()
            .add_event::<PlaySfxEvent>()
            .add_audio_channel::<BgmChannel>()
            .add_audio_channel::<SfxChannel>();
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum BGM {
    Title,
    Editor,
    Level,
}

impl BGM {
    fn get_handle(&self, audio_assets: &Res<AudioAssets>) -> Handle<AudioSource> {
        match self {
            BGM::Title => audio_assets.title.clone(),
            BGM::Editor => audio_assets.editor.clone(),
            BGM::Level => audio_assets.level.clone(),
        }
    }
}

#[derive(Copy, Clone)]
pub enum SFX {
    Clic,
    Error,
    Place,
    Win,
}

impl SFX {
    fn get_handle(&self, audio_assets: &Res<AudioAssets>) -> Handle<AudioSource> {
        match self {
            SFX::Clic => audio_assets.clic.clone(),
            SFX::Error => audio_assets.error.clone(),
            SFX::Place => audio_assets.place.clone(),
            SFX::Win => audio_assets.win.clone(),
        }
    }
}

#[derive(Resource)]
pub struct BgmChannel;

#[derive(Resource)]
pub struct SfxChannel;

pub struct PlayBgmEvent(pub BGM);

pub struct PlaySfxEvent(pub SFX);

#[derive(Resource)]
struct CurrentBGM(BGM);

fn update_bgm(
    mut bgm_events: EventReader<PlayBgmEvent>,
    mut sfx_events: EventReader<PlaySfxEvent>,
    audio_assets: Option<Res<AudioAssets>>,
    bgm_channel: Res<AudioChannel<BgmChannel>>,
    sfx_channel: Res<AudioChannel<SfxChannel>>,
    mut commands: Commands,
    current: Option<Res<CurrentBGM>>,
) {
    if audio_assets.is_none() { return; }

    // Play BGMs
    for PlayBgmEvent(bgm) in bgm_events.iter() {
        if let Some(c) = current {
            if c.0 == *bgm { return; }
        }

        commands.insert_resource(CurrentBGM(*bgm));
        bgm_channel.stop();
        bgm_channel.set_volume(0.4);
        bgm_channel
            .play(bgm.get_handle(&audio_assets.as_ref().unwrap()))
            .looped();
        break;
    }
    bgm_events.clear();

    // Play SFXs
    for PlaySfxEvent(sfx) in sfx_events.iter() {
        sfx_channel.set_volume(0.3);
        sfx_channel.play(sfx.get_handle(&audio_assets.as_ref().unwrap()));
    }
}