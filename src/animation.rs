use std::time::Duration;
use bevy::prelude::*;

pub struct Animation {
    pub first: usize,
    pub last: usize,
    pub texture: Handle<TextureAtlas>,
    pub frame_rate: Duration,
}
