use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::{GameState, HEIGHT, util};
use crate::loading::Textures;
use crate::puzzle::Puzzle;
use crate::veggie::Veggie;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentPuzzle(None))
            .add_event::<DisplayLevel>()
            .add_event::<DestroyLevel>()
            .add_system_set(SystemSet::on_enter(GameState::Puzzle).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Puzzle)
                .with_system(update)
                .with_system(display_level)
            )
            .add_system_set(SystemSet::on_exit(GameState::Puzzle).with_system(cleanup));
    }
}

#[derive(Component)]
struct GridUI;

#[derive(Resource)]
struct CurrentPuzzle(pub Option<Puzzle>);

struct DisplayLevel;
struct DestroyLevel;

fn setup(
    mut puzzle: ResMut<CurrentPuzzle>,
    mut display_level: EventWriter<DisplayLevel>,
) {
    // TODO: Load real level
    puzzle.0 = Some(Puzzle {
        size: (4, 3),
        veggies: vec![(Veggie::Strawberry, 2)],
        tiles: Default::default(),
    });

    display_level.send(DisplayLevel);
}

fn display_level(
    mut commands: Commands,
    textures: Res<Textures>,
    mut ev: EventReader<DisplayLevel>,
    puzzle: Res<CurrentPuzzle>,
) {
    for _ in ev.iter() {
        if let Some(puzzle) = &puzzle.0 {
            let h = (HEIGHT / 2. - puzzle.size.1 as f32 * 40.) / 2.;
            let w = 144.;

            // Tiles
            for y in 0..puzzle.size.1 {
                for x in 0..puzzle.size.0 {
                    commands
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                anchor: Anchor::BottomLeft,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(w + x as f32 * 40., h + y as f32 * 40., util::z::TILE),
                            texture: textures.tile.clone(),
                            ..Default::default()
                        })
                        .insert(GridUI);
                }
            }

            // Corners
            for (dx, dy, fx, fy, dx2, dy2, sx, sy, i) in [
                (-8., puzzle.size.1 as f32 * 40., false, false, 8., 0., puzzle.size.0 as f32 * 5., 1., 1),
                (puzzle.size.0 as f32 * 40., puzzle.size.1 as f32 * 40., true, false, 0., -1. * puzzle.size.1 as f32 * 40., 1., puzzle.size.1 as f32 * 5., 2),
                (-8., -8., false, true, 8., 0., puzzle.size.0 as f32 * 5., 1., 3),
                (puzzle.size.0 as f32 * 40., -8., true, true, -1. * puzzle.size.0 as f32 * 40. - 8., 8., 1., puzzle.size.1 as f32 * 5., 4),
            ] {
                commands
                    .spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: 0,
                            flip_x: fx,
                            flip_y: fy,
                            anchor: Anchor::BottomLeft,
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(w + dx, h + dy, util::z::TILE),
                        texture_atlas: textures.border.clone(),
                        ..Default::default()
                    })
                    .insert(GridUI);

                commands
                    .spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: i,
                            anchor: Anchor::BottomLeft,
                            ..Default::default()
                        },
                        transform: Transform {
                            translation: Vec3::new(w + dx + dx2, h + dy + dy2, util::z::TILE),
                            scale: Vec3::new(sx, sy, 1.),
                            ..Default::default()
                        },
                        texture_atlas: textures.border.clone(),
                        ..Default::default()
                    })
                    .insert(GridUI);
            }
        }
    }
}

fn update() {}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<GridUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}