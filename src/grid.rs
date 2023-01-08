use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::hashbrown::HashMap;
use crate::{GameState, HEIGHT, puzzle, util, WIDTH};
use crate::inventory::DraggedVeg;
use crate::loading::Textures;
use crate::puzzle::{Puzzle, Tile};
use crate::veggie::{Expression, spawn_veggie, UpdateFaces, Veggie};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentPuzzle(None))
            .add_event::<DisplayLevel>()
            .add_event::<DestroyLevel>()
            .add_event::<GridChanged>()
            .add_system_set(SystemSet::on_enter(GameState::Puzzle)
                .with_system(setup)
            )
            .add_system_set(SystemSet::on_update(GameState::Puzzle)
                .with_system(update.before("logic"))
                .with_system(display_level.label("logic"))
                .with_system(handle_click.label("logic"))
            )
            .add_system_set(SystemSet::on_exit(GameState::Puzzle).with_system(cleanup));
    }
}

#[derive(Component)]
pub struct GridUI;

#[derive(Resource)]
pub struct CurrentPuzzle(pub Option<Puzzle>);

pub struct DisplayLevel;
struct DestroyLevel;

pub struct GridChanged;

#[derive(Component)]
pub struct GridVeggie(pub Veggie, pub (i8, i8), pub (bool, bool));

fn setup(
    mut puzzle: ResMut<CurrentPuzzle>,
    mut display_level: EventWriter<DisplayLevel>,
) {
    // TODO: Load real level
    puzzle.0 = Some(Puzzle {
        author: "yopox".to_string(),
        size: (5, 3),
        veggies: HashMap::new(),
        tiles: HashMap::new(),
        placed: HashMap::new(),
    });

    display_level.send(DisplayLevel);
}

#[derive(Component)]
pub struct GridTile(pub Tile, pub (i8, i8));

fn display_level(
    mut commands: Commands,
    textures: Res<Textures>,
    mut ev: EventReader<DisplayLevel>,
    puzzle: Res<CurrentPuzzle>,
    entities: Query<Entity, With<GridUI>>,
    mut grid_changed: EventWriter<GridChanged>,
) {
    for _ in ev.iter() {
        // "Reset grid"
        entities.iter().for_each(|e| commands.entity(e).despawn_recursive());

        if let Some(puzzle) = &puzzle.0 {
            let h = (HEIGHT - puzzle.size.1 as f32 * 40.) / 2.;
            let w = (WIDTH - puzzle.size.0 as f32 * 40.) / 2.;

            // Tiles
            for y in 0..puzzle.size.1 {
                for x in 0..puzzle.size.0 {
                    let tile_x = w + x as f32 * 40.;
                    let tile_y = h + y as f32 * 40.;

                    commands
                        .spawn(SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                index: ((x + y) % 2) as usize,
                                anchor: Anchor::BottomLeft,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(tile_x, tile_y, util::z::TILE),
                            texture_atlas: textures.tile.clone(),
                            ..Default::default()
                        })
                        .insert(GridUI);

                    if let Some(tile) = puzzle.tiles.get(&(x, y)) {
                        commands
                            .spawn(SpriteSheetBundle {
                                sprite: TextureAtlasSprite {
                                    index: tile.index(),
                                    anchor: Anchor::BottomLeft,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(tile_x, tile_y, util::z::TILE_ABOVE),
                                texture_atlas: textures.tile.clone(),
                                ..Default::default()
                            })
                            .insert(GridTile(tile.clone(), (x, y)))
                            .insert(GridUI);
                    }

                    if let Some(veggie) = puzzle.placed.get(&(x, y)) {
                        let id = spawn_veggie(
                            &mut commands,
                            &textures,
                            Vec3::new(tile_x, tile_y, util::z::VEGGIE),
                            veggie,
                            Expression::Neutral,
                        );
                        commands
                            .entity(id)
                            .insert(GridVeggie(veggie.clone(), (x, y), (false, false)))
                            .insert(GridUI);
                    }
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

            grid_changed.send(GridChanged);
        }
    }
}

pub fn get_pos_at(cursor_pos: Vec2, puzzle_size: (i8, i8)) -> Option<(i8, i8)> {
    let (x, y) = (cursor_pos.x / 2., cursor_pos.y / 2.);

    let h = (HEIGHT - puzzle_size.1 as f32 * 40.) / 2.;
    let w = (WIDTH - puzzle_size.0 as f32 * 40.) / 2.;

    let t_x = (x - w) / 40.;
    let t_y = (y - h) / 40.;

    if t_x > 0. && t_x < puzzle_size.0 as f32 && t_y > 0. && t_y < puzzle_size.1 as f32 {
        return Some((t_x as i8, t_y as i8));
    }
    return None;
}

pub fn get_tile_pos(tile: (i8, i8), puzzle_size: (i8, i8)) -> Vec2 {
    let h = (HEIGHT - puzzle_size.1 as f32 * 40.) / 2.;
    let w = (WIDTH - puzzle_size.0 as f32 * 40.) / 2.;

    return Vec2::new(w + tile.0 as f32 * 40., h + tile.1 as f32 * 40.);
}

fn update(
    mut changed: EventReader<GridChanged>,
    mut veggies: Query<(&mut GridVeggie, Entity)>,
    mut update_faces: EventWriter<UpdateFaces>,
    puzzle: Res<CurrentPuzzle>,
) {
    for _ in changed.iter() {
        if puzzle.0.is_none() { return; }
        let puzzle = puzzle.0.as_ref().unwrap();

        for (mut veg, e) in veggies.iter_mut() {
            let state = puzzle::is_happy(&veg.0, veg.1, &puzzle.tiles, &puzzle.placed);
            veg.2 = state;
            let exp = |b| if b { Expression::Happy } else { Expression::Sad };
            update_faces.send(UpdateFaces(e, (exp(state.0), exp(state.1))));
        }
    }
}

#[derive(Component)]
pub struct PreviousPos(pub (i8, i8));

fn handle_click(
    mut commands: Commands,
    mut veggies: Query<(Entity, &GridVeggie, &mut Transform)>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut puzzle: ResMut<CurrentPuzzle>,
    mut update_faces: EventWriter<UpdateFaces>,
    mut grid_changed: EventWriter<GridChanged>,
) {
    if puzzle.0.is_none() { return; }
    let mut puzzle = puzzle.0.as_mut().unwrap();

    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            if let Some((e, v, mut t)) = veggies.iter_mut().filter(|(_, _, t)|
                (t.translation.x + 20. - pos.x / 2.).abs() < 20.
                    && (t.translation.y + 20. - pos.y / 2.).abs() < 20.
            ).nth(0) {
                commands
                    .entity(e)
                    .insert(DraggedVeg(v.0))
                    .insert(PreviousPos(v.1))
                    .remove::<GridVeggie>();
                puzzle.placed.remove(&v.1);
                t.translation.z = util::z::VEG_DRAG;
                grid_changed.send(GridChanged);
                update_faces.send(UpdateFaces(e, (Expression::Surprised, Expression::Surprised)));
            }
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<GridUI>>,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}