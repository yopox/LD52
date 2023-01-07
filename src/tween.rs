use std::time::Duration;

use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;
use bevy_tweening::{EaseFunction, EaseMethod, Lens, Tween};
use bevy_tweening::EaseMethod::Linear;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};

pub fn position_out(start: Vec2, end: Vec2, z: f32, time: u64) -> Tween<Transform> {
    position(EaseFunction::CircularOut, start, end, z, time)
}

pub fn position_in(start: Vec2, end: Vec2, z: f32, time: u64) -> Tween<Transform> {
    position(EaseFunction::CircularIn, start, end, z, time)
}

pub fn position(ease: impl Into<EaseMethod>, start: Vec2, end: Vec2, z: f32, time: u64) -> Tween<Transform> {
    Tween::new(
        ease,
        Duration::from_millis(time),
        TransformPositionLens {
            start: start.extend(z),
            end: end.extend(z),
        },
    )
}

pub fn scale(start: f32, end: f32, time: u64) -> Tween<Transform> {
    Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_millis(time),
        TransformScaleLens { start: Vec3::new(start, start, 1.), end: Vec3::new(end, end, 1.) },
    )
}

pub fn linear_scale(start: f32, end: f32, time: u64) -> Tween<Transform> {
    Tween::new(
        Linear,
        Duration::from_millis(time),
        TransformScaleLens { start: Vec3::new(start, start, 1.), end: Vec3::new(end, end, 1.) },
    )
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformSpriteAlphaLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<Sprite> for TransformSpriteAlphaLens {
    fn lerp(&mut self, target: &mut Sprite, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.color.set_a(value);
    }
}

pub fn tween_sprite_opacity(ms: u64, appear: bool) -> Tween<Sprite> {
    Tween::new(
        EaseFunction::CubicOut,
        Duration::from_millis(ms),
        TransformSpriteAlphaLens {
            start: if appear { 0. } else { 1. },
            end: if appear { 1. } else { 0. },
        }
    )
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformTextureAtlasSpriteAlphaLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<TextureAtlasSprite> for TransformTextureAtlasSpriteAlphaLens {
    fn lerp(&mut self, target: &mut TextureAtlasSprite, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.color.set_a(value);
    }
}

pub fn tween_texture_atlas_sprite_opacity(ms: u64, appear: bool) -> Tween<TextureAtlasSprite> {
    Tween::new(
        EaseFunction::CubicOut,
        Duration::from_millis(ms),
        TransformTextureAtlasSpriteAlphaLens {
            start: if appear { 0. } else { 1. },
            end: if appear { 1. } else { 0. },
        }
    )
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformTextModeSpriteAlphaLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<TextModeTextureAtlasSprite> for TransformTextModeSpriteAlphaLens {
    fn lerp(&mut self, target: &mut TextModeTextureAtlasSprite, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.alpha = value;
    }
}

pub fn tween_text_mode_sprite_opacity(ms: u64, appear: bool) -> Tween<TextModeTextureAtlasSprite> {
    Tween::new(
        EaseFunction::CubicOut,
        Duration::from_millis(ms),
        TransformTextModeSpriteAlphaLens {
            start: if appear { 0. } else { 1. },
            end: if appear { 1. } else { 0. },
        }
    )
}