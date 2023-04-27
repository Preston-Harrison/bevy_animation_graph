use std::time::Duration;

use bevy::prelude::*;

pub struct Animation {
    first: usize,
	last: usize,
    texture: Handle<TextureAtlas>,
    frame_rate: Duration,
}

impl Animation {
    pub fn new(
        first: usize,
        last: usize,
        texture: Handle<TextureAtlas>,
        frame_rate: Duration,
    ) -> Self {
        Animation {
            first,
			last,
            texture,
            frame_rate,
        }
    }

	pub fn first(&self) -> usize {
		self.first
	}

	pub fn last(&self) -> usize {
		self.last
	}
}