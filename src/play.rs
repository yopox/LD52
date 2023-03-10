use bevy::prelude::*;
use bevy_pkv::PkvStore;
use bevy_tweening::Animator;

use crate::{BlockInput, GameState, grid, HEIGHT, text, tween, util, WIDTH};
use crate::audio::{BGM, PlayBgmEvent, PlaySfxEvent, SFX};
use crate::grid::{CurrentPuzzle, DisplayLevel, GridChanged};
use crate::loading::Textures;
use crate::overworld::{CurrentSlot, Slot};
use crate::progress::{get_progress, set_progress};
use crate::text::{ButtonClick, TextButtonId};
use crate::util::Colors;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Play)
                .with_system(play_music)
            )
            .add_system_set(SystemSet::on_update(GameState::Play)
                .with_system(display)
                .with_system(click_on_button)
                .with_system(check_finished)
                .with_system(win_animation)
            )
            .add_system_set(SystemSet::on_exit(GameState::Play).with_system(cleanup));
    }
}

#[derive(Component)]
struct PlayUI;

fn play_music(
    mut bgm: EventWriter<PlayBgmEvent>,
) {
    bgm.send(PlayBgmEvent(BGM::Level));
}

fn display(
    mut commands: Commands,
    textures: Res<Textures>,
    puzzle: Res<CurrentPuzzle>,
    mut events: EventReader<DisplayLevel>,
    slot: Option<Res<CurrentSlot>>,
) {
    if puzzle.0.is_none() { return; }
    let puzzle = puzzle.0.as_ref().unwrap();

    for _ in events.iter() {
        // Level details
        let x = WIDTH - 32. - 48. - 24.;
        let h = 7;
        let y = (HEIGHT - (h + 5) as f32 * 8.) / 2.;

        // Frame
        let id = util::frame(
            &mut commands, &textures,
            x, y + 5. * 8., util::z::VEG_UI_BG,
            11, h,
            Colors::DarkRed, Colors::Beige
        );
        commands.entity(id).insert(PlayUI);

        // Level number
        let text = match slot {
            Some(s) => match s.0 {
                Slot::Level(n) => format!("level\n#{:0>2}", n + 1),
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };
        let text_x = x + 8.;
        let id = text::spawn_text(
            &mut commands, &textures,
            Vec3::new(text_x, y + 10. * 8., util::z::VEG_UI),
            if text.is_empty() { "custom\nlevel" } else { &text },
            Colors::Beige, Colors::DarkRed,
        );
        commands.entity(id).insert(PlayUI);

        // Level author
        let id = text::spawn_text(
            &mut commands, &textures,
            Vec3::new(text_x, y + 7. * 8., util::z::VEG_UI),
            &format!("by:\n{}", if puzzle.author.is_empty() { "unknown" } else { &puzzle.author }),
            Colors::Beige, Colors::DarkRed,
        );
        commands.entity(id).insert(PlayUI);

        // Exit button
        let id = util::frame(
            &mut commands, &textures,
            x, y, util::z::VEG_UI_BG,
            11, 3,
            Colors::DarkRed, Colors::Beige
        );
        commands.entity(id).insert(PlayUI);

        let id = text::spawn_text(
            &mut commands, &textures,
            Vec3::new(text_x, y + 16., util::z::VEG_UI),
            &"         \n- leave -\n         ",
            Colors::Beige, Colors::DarkRed,
        );
        commands.entity(id)
            .insert(PlayUI)
            .insert(TextButtonId::LeaveLevel);

        break;
    }
    events.clear();
}

fn click_on_button(
    mut clicks: EventReader<ButtonClick>,
    mut state: ResMut<State<GameState>>,
    block_input: Res<BlockInput>,
) {
    if block_input.0 { return; }

    for id in clicks.iter() {
        match id.0 {
            TextButtonId::LeaveLevel => {
                state.pop().unwrap_or_default();
            }

            _ => {}
        }
    }
}

fn check_finished(
    mut commands: Commands,
    mut changed: EventReader<GridChanged>,
    puzzle: Res<CurrentPuzzle>,
    mut block_input: ResMut<BlockInput>,
    slot: Option<Res<CurrentSlot>>,
    mut pkv: ResMut<PkvStore>,
    mut sfx: EventWriter<PlaySfxEvent>,
) {
    if puzzle.0.is_none() { return; }
    let puzzle = puzzle.0.as_ref().unwrap();

    for _ in changed.iter() {
        if puzzle.veggies.iter().all(|v| puzzle.remaining_veggie(v.0, false) == 0)
            && puzzle.is_valid().is_ok() {
            block_input.0 = true;
            commands.insert_resource(WinAnimation { n: 0, frame: 0, global: 0 });

            sfx.send(PlaySfxEvent(SFX::Win));

            if let Some(s) = &slot {
                let mut progress = get_progress(pkv.as_ref());
                match s.0 {
                    Slot::Level(n) => { progress.finished_levels.insert(n); },
                    Slot::Custom(n) => { progress.finished_custom.insert(n); },
                    _ => {},
                };
                set_progress(pkv.as_mut(), &progress);
            }
        }
    }
}

#[derive(Resource)]
struct WinAnimation {
    n: usize,
    frame: u8,
    global: usize,
}

fn win_animation(
    mut commands: Commands,
    textures: Res<Textures>,
    mut animation: Option<ResMut<WinAnimation>>,
    mut state: ResMut<State<GameState>>,
    mut block_input: ResMut<BlockInput>,
    puzzle: Res<CurrentPuzzle>,
) {
    if animation.is_none() || puzzle.0.is_none() { return; }
    let mut binding = animation.unwrap();
    let animation = binding.as_mut();
    let puzzle = puzzle.0.as_ref().unwrap();

    if animation.frame == 0 && animation.n < puzzle.placed.len() {
        let veg = puzzle.placed.iter().nth(animation.n).unwrap();
        let pos = grid::get_tile_pos(*veg.0, puzzle.size);
        commands
            .spawn(SpriteBundle {
                texture: textures.heart.clone(),
                transform: Transform::from_xyz(pos.x + 8., pos.y + 24., util::z::WIN_HEART),
                ..Default::default()
            })
            .insert(Animator::new(tween::position_out(
                Vec2::new(pos.x + 8., pos.y + 24.),
                Vec2::new(pos.x + 8., pos.y + 32.),
                util::z::WIN_HEART,
                1000
            )))
            .insert(Animator::new(tween::tween_sprite_opacity(800, false)))
            .insert(PlayUI);
    }

    animation.frame += 1;
    animation.global += 1;

    if animation.n >= puzzle.placed.len() {
        if animation.frame > 20 && animation.global > 210 {
            commands.remove_resource::<WinAnimation>();
            block_input.0 = false;
            state.pop().unwrap();
            return;
        }
    } else if animation.frame == 15 {
        animation.frame = 0;
        animation.n += 1;
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<PlayUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}