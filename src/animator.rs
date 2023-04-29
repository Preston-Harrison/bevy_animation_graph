use crate::{frame_iterator::FrameIterator, state_graph::StateGraph};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub struct Animation {
    pub first_frame: usize,
    pub last_frame: usize,
    pub texture: Handle<TextureAtlas>,
    pub frame_duration: Duration,
}

#[derive(Clone)]
pub struct AnimationData {
    pub animation: Animation,
    pub has_exit_time: bool,
}

impl AnimationData {
    pub fn new(animation: Animation, has_exit_time: bool) -> Self {
        AnimationData {
            animation,
            has_exit_time,
        }
    }
}

#[derive(Component)]
pub struct Animator {
    pub state_graph: StateGraph<AnimationData>,
    pub frame_iterator: Option<FrameIterator>,
    pub last_frame_time: Option<Duration>,
    pub play_after_exit: Option<String>,
}

impl Animator {
    pub fn new(state_graph: StateGraph<AnimationData>) -> Self {
        Animator {
            state_graph,
            frame_iterator: None,
            last_frame_time: None,
            play_after_exit: None,
        }
    }

    pub fn play(&mut self, name: String) {
        if *self.state_graph.get_active().0 == name {
            return;
        }
    }
}
