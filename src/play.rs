use bevy::prelude::*;

use crate::{GameState, HEIGHT, text, util, WIDTH};
use crate::editor::InEditor;
use crate::grid::{CurrentPuzzle, DisplayLevel};
use crate::loading::Textures;
use crate::text::{ButtonClick, TextButtonId};
use crate::util::Colors;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Puzzle)
                .with_system(display)
                .with_system(click_on_button)
            )
            .add_system_set(SystemSet::on_exit(GameState::Puzzle).with_system(cleanup));
    }
}

#[derive(Component)]
struct PlayUI;

fn display(
    mut commands: Commands,
    textures: Res<Textures>,
    puzzle: Res<CurrentPuzzle>,
    mut events: EventReader<DisplayLevel>,
    in_editor: Res<InEditor>,
) {
    if puzzle.0.is_none() || in_editor.0 { return; }
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
        let text_x = x + 8.;
        let id = text::spawn_text(
            &mut commands, &textures,
            Vec3::new(text_x, y + 10. * 8., util::z::VEG_UI),
            &format!("level\n#01"),
            Colors::Beige.get(), Colors::DarkRed.get(),
        );
        commands.entity(id).insert(PlayUI);

        // Level author
        let id = text::spawn_text(
            &mut commands, &textures,
            Vec3::new(text_x, y + 7. * 8., util::z::VEG_UI),
            &format!("by:\n{}", if puzzle.author.is_empty() { "unknown" } else { &puzzle.author }),
            Colors::Beige.get(), Colors::DarkRed.get(),
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
            Colors::Beige.get(), Colors::DarkRed.get(),
        );
        commands.entity(id)
            .insert(PlayUI)
            .insert(TextButtonId::Exit);

        break;
    }
    events.clear();
}

fn click_on_button(
    mut clicks: EventReader<ButtonClick>,
    in_editor: Res<InEditor>,
    mut state: ResMut<State<GameState>>,
) {
    if in_editor.0 { return; }

    for id in clicks.iter() {
        match id.0 {
            TextButtonId::Exit => {
                state.set(GameState::Title).unwrap_or_default();
            }

            _ => {}
        }
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