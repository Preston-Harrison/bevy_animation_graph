use bevy::prelude::*;
use std::time::Duration;

pub struct Animation {
    pub first: usize,
    pub last: usize,
    pub texture: Handle<TextureAtlas>,
    pub frame_rate: Duration,
    pub has_exit_time: bool,
}
