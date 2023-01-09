use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{GameState, progress, text, util, WIDTH};
use crate::loading::Textures;
use crate::overworld::{CurrentSlot, Slot};
use crate::text::{ButtonClick, TextButtonId};
use crate::util::Colors;
use crate::veggie::{Expression, spawn_veggie, Veggie};

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Tutorial).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Tutorial).with_system(click_on_button))
            .add_system_set(SystemSet::on_exit(GameState::Tutorial).with_system(cleanup));
    }
}

#[derive(Component)]
struct TutorialUI;

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    mut pkv: ResMut<PkvStore>,
    slot: Option<Res<CurrentSlot>>,
) {
    let pages = [
        ([
             (Veggie::Strawberry, 3, 8, Expression::Happy),
             (Veggie::Strawberry, 7, 8, Expression::Happy),
             (Veggie::Strawberry, 5, 20, Expression::Sad),
         ],
         "it's winter, time to plan our future garden\n\
         to get the best harvest possible!\n\n\
         on the left you will find the veggies\n\
         that must be planted and their count.\n\n\
         drag them on an empty tile of the grid.\n\n\
         the first veggie is the strawberry:\n\
         it likes being adjacent (diagonals count)\n\
         to another strawberry."),
    ];

    if let Some(s) = slot {
        let page = match s.0 {
            Slot::Tutorial(n) => {
                let mut progress = progress::get_progress(pkv.as_ref());
                progress.tutorial.insert(n);
                progress::set_progress(pkv.as_mut(), &progress);
                &pages[n as usize]
            },
            _ => &pages[0],
        };

        for (veg, x, y, e) in &page.0 {
            let id = spawn_veggie(
                &mut commands, &textures,
                Vec3::new(64. + *x as f32 * 8., 80. + *y as f32 * 8., util::z::VEG_UI),
                &veg,
                *e
            );
            commands.entity(id).insert(TutorialUI);
        }

        let id = util::frame(
            &mut commands, &textures,
            64., 80., util::z::VEG_UI_BG,
            64, 28,
            Colors::DarkRed, Colors::Beige
        );
        commands.entity(id).insert(TutorialUI);

        let id = text::spawn_text(
            &mut commands, &textures,
            Vec3::new(128. + 64., 240., util::z::VEG_UI),
            page.1,
            Colors::Beige, Colors::DarkRed
        );
        commands.entity(id).insert(TutorialUI);

    }

    let id = util::frame(
        &mut commands, &textures,
        WIDTH - 120. + 8., 24., util::z::VEG_UI_BG,
        11, 3,
        Colors::DarkRed, Colors::Beige
    );
    commands.entity(id).insert(TutorialUI);

    let id = text::spawn_text(
        &mut commands, &textures,
        Vec3::new(WIDTH - 112. + 8., 40., util::z::VEG_UI),
        &"        \n- okay -\n        ",
        Colors::Beige, Colors::DarkRed,
    );
    commands.entity(id)
        .insert(TextButtonId::Exit)
        .insert(TutorialUI);
}

fn click_on_button(
    mut clicked: EventReader<ButtonClick>,
    mut state: ResMut<State<GameState>>,
) {
    for ButtonClick(id) in clicked.iter() {
        match *id {
            TextButtonId::Tutorial(n) => match n {
                _ => {}
            }

            TextButtonId::Exit => {
                state.set(GameState::Overworld).unwrap();
            }

            _ => {}
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<TutorialUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}