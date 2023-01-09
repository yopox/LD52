use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{GameState, levels, text, util};
use crate::data::Decoder;
use crate::grid::CurrentPuzzle;
use crate::loading::Textures;
use crate::progress::get_progress;
use crate::text::{ButtonClick, TextButtonId};
use crate::util::Colors;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(setup))
            .add_system_set(SystemSet::on_resume(GameState::Overworld).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Overworld)
                .with_system(click_on_button)
            )
            .add_system_set(SystemSet::on_exit(GameState::Overworld).with_system(cleanup))
            .add_system_set(SystemSet::on_pause(GameState::Overworld).with_system(cleanup))
        ;
    }
}

#[derive(Component)]
struct OverworldUI;

#[derive(Resource)]
pub struct CurrentSlot(pub Slot);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Slot {
    Level(usize),
    Tutorial(u8),
    Custom(usize),
}

impl Slot {
    fn char(&self) -> char {
        match self {
            Slot::Level(_) => 'L',
            Slot::Tutorial(_) => 'T',
            Slot::Custom(_) => 'C',
            _ => ' ',
        }
    }
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    pkv: Res<PkvStore>,
) {
    let progress = get_progress(pkv.as_ref());

    let id = util::frame(
        &mut commands, &textures,
        32., 256. + 48., util::z::VEG_UI_BG,
        8, 3,
        Colors::DarkRed, Colors::Beige,
    );
    commands.entity(id).insert(OverworldUI);

    let id = text::spawn_text(
        &mut commands, &textures,
        Vec3::new(40., 264. + 48., util::z::VEG_UI),
        "levels",
        Colors::Beige, Colors::DarkRed,
    );
    commands.entity(id).insert(OverworldUI);

    let id = util::frame(
        &mut commands, &textures,
        48., 256. - 80., util::z::VEG_UI_BG,
        34, 14,
        Colors::DarkRed, Colors::Beige,
    );
    commands.entity(id).insert(OverworldUI);

    let mut tile_x = 8;
    let mut tile_y = 32;
    for (slot, arrow) in [
        (Slot::Tutorial(0), 3),
        (Slot::Level(0), 3),
        (Slot::Tutorial(1), 3),
        (Slot::Level(1), 3),
        (Slot::Tutorial(2), 3),
        (Slot::Level(2), 3),
        (Slot::Tutorial(3), 3),
        (Slot::Level(3), 2),
        (Slot::Tutorial(4), 1),
        (Slot::Level(4), 1),
        (Slot::Level(5), 1),
        (Slot::Level(6), 1),
        (Slot::Tutorial(5), 1),
        (Slot::Level(7), 1),
        (Slot::Level(8), 1),
        (Slot::Tutorial(6), 2),
        (Slot::Level(9), 3),
        (Slot::Level(10), 3),
        (Slot::Level(11), 3),
        (Slot::Level(12), 3),
    ] {
        let completed = match slot {
            Slot::Level(n) => progress.finished_levels.contains(&n),
            Slot::Custom(n) => progress.custom_levels.get(n).unwrap().1,
            Slot::Tutorial(n) => progress.tutorial.contains(&n),
        };
        let color = if completed { Colors::Green } else { Colors::Red };

        let id = text::spawn_text(
            &mut commands, &textures,
            Vec3::new(tile_x as f32 * 8., tile_y as f32 * 8., util::z::VEG_UI),
            &slot.char().to_string(),
            Colors::Beige, color,
        );
        commands
            .entity(id)
            .insert(TextButtonId::Overworld(slot))
            .insert(OverworldUI);

        let char = match arrow {
            0 => Some(('W', 0, 2)),
            1 => Some(('A', -2, 0)),
            2 => Some(('S', 0, -2)),
            3 => Some(('D', 2, 0)),
            _ => None,
        };
        if let Some((c, dx, dy)) = char {
            tile_x += dx;
            tile_y += dy;
            let id = text::spawn_text(
                &mut commands, &textures,
                Vec3::new(tile_x as f32 * 8., tile_y as f32 * 8., util::z::VEG_UI),
                &c.to_string(),
                Colors::Beige, Colors::DarkRed,
            );
            commands.entity(id).insert(OverworldUI);
            tile_x += dx;
            tile_y += dy;
        }
    }
}

fn click_on_button(
    mut commands: Commands,
    mut clicked: EventReader<ButtonClick>,
    mut state: ResMut<State<GameState>>,
    mut current_puzzle: ResMut<CurrentPuzzle>,
) {
    for ButtonClick(id) in clicked.iter() {
        match *id {
            TextButtonId::Overworld(slot) => match slot {
                Slot::Level(n) => {
                    if let Some(mut puzzle) = Decoder::decode_puzzle(levels::LEVELS[n].to_string()) {
                        puzzle.prepare();
                        current_puzzle.as_mut().0 = Some(puzzle);
                        commands.insert_resource(CurrentSlot(slot));
                        state.push(GameState::Play).unwrap();
                    }
                }
                _ => {}
            }

            TextButtonId::Exit => {
                state.pop().unwrap();
            }

            _ => {}
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<OverworldUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}