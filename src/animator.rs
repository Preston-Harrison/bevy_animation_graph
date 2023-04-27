use bevy::prelude::*;
use std::{collections::HashMap, time::Duration};

use crate::{animation::Animation, texture_iterator::TextureIterator};

pub struct Animator {
    animations: HashMap<String, Animation>,
    default_animation: Animation,
    active_animation: Option<String>,
    frame: TextureIterator,
    frame_start: Duration,
}

impl Animator {
    pub fn new(default_animation: Animation, current_time: Res<Time>) -> Self {
        Animator {
            animations: HashMap::default(),
            frame: TextureIterator::new(default_animation.first(), default_animation.last()),
            default_animation,
            active_animation: None,
            frame_start: current_time.elapsed(),
        }
    }

	pub fn add_animation(&mut self, name: String, animation: Animation) {
		// TODO not panic here
		assert!(!self.animations.contains_key(&name));

		self.animations.insert(name, animation);
	}
}
