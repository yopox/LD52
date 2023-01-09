use std::ops::Add;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

use crate::loading::Textures;
use crate::overworld::Slot;
use crate::util::{collides, Colors};

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ChangeText>()
            .add_event::<ButtonClick>()
            .add_system(update_text)
            .add_system(handle_click)
        ;
    }
}

#[derive(Component)]
pub struct Text(pub Vec<String>);

#[derive(Component)]
struct CharId(usize, usize);

pub fn spawn_text<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    textures: &Res<Textures>,
    position: Vec3,
    text: &str,
    bg: Colors,
    fg: Colors,
) -> Entity {
    let lines = cut_str(text);
    commands
        .spawn(Text(lines.clone()))
        .insert(Transform::from_translation(position))
        .insert(Visibility::default())
        .insert(GlobalTransform::default())
        .insert(ComputedVisibility::default())
        .with_children(|parent| {
            // Spawn letters
            for line_n in 0..lines.len() {
                for (char_n, c) in lines.get(line_n).unwrap().chars().enumerate() {
                    parent
                        .spawn(TextModeSpriteSheetBundle {
                            sprite: TextModeTextureAtlasSprite {
                                bg: bg.get(),
                                fg: fg.get(),
                                index: char_to_index(c),
                                anchor: Anchor::BottomLeft,
                                ..Default::default()
                            },
                            texture_atlas: textures.mrmotext.clone(),
                            transform: Transform::from_xyz(char_n as f32 * 8., line_n as f32 * 8. * -1., 0.),
                            ..Default::default()
                        })
                        .insert(CharId(line_n, char_n));
                }
            }
        })
        .id()
}

fn cut_str(text: &str) -> Vec<String> {
    text.split("\n").map(|s| s.to_string()).collect::<Vec<String>>()
}

fn char_to_index(c: char) -> usize {
    return match c {
        'a'..='z' => 28 * 32 + 1 + c as u32 - 'a' as u32,
        '!'..='?' => 27 * 32 + 1 + c as u32 - '!' as u32,
        _ => 0,
    } as usize;
}

pub struct ChangeText(pub Entity, pub String);

fn update_text(
    mut events: EventReader<ChangeText>,
    mut text_query: Query<(&mut Text, Entity)>,
    mut tile_query: Query<(&mut TextModeTextureAtlasSprite, &CharId, &Parent)>,
) {
    for ChangeText(e, t) in events.iter() {
        let lines = cut_str(t);

        if let Ok((mut text, e)) = text_query.get_mut(*e) {
            if text.0.join("\n") == lines.join("\n") { continue }

            let mut tiles = tile_query
                .iter_mut()
                .filter(|(_, _, p)| p.get() == e)
                .collect::<Vec<(Mut<'_, TextModeTextureAtlasSprite>, &CharId, &Parent)>>();

            for line_n in 0..lines.len() {
                for (char_n, c) in lines.get(line_n).unwrap().chars().enumerate() {
                    if let Some((tile, _, _)) = tiles.iter_mut().filter(|(_, id, _)| id.0 == line_n && id.1 == char_n).nth(0) {
                        // Update existing char
                        tile.index = char_to_index(c);
                    }
                }
            }

            text.0 = lines;
        }
    }
}

#[derive(Component, Eq, PartialEq, Clone)]
pub enum TextButtonId {
    ExpandShrink(bool, bool),
    Export,
    Import,
    Clear,
    Exit,
    Title(u8),
}

pub struct ButtonClick(pub TextButtonId);

fn handle_click(
    buttons: Query<(&Text, &Transform, &TextButtonId)>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut ev: EventWriter<ButtonClick>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();
        if let Some(pos) = window.cursor_position() {
            if let Some((_, _, id)) = buttons.iter().filter(|(text, t, _)|
                collides(t.translation.add(Vec3::new(0., (text.0.len() - 1) as f32 * 8. * -1., 0.)), text.0.get(0).unwrap().len() as f32 * 8., text.0.len() as f32 * 8., pos)
            ).nth(0) {
                ev.send(ButtonClick(id.clone()));
            }
        }
    }
}