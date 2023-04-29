use std::time::Duration;

use crate::{state_graph::StateGraph, animation::Animation, texture_iterator::TextureIterator};
use bevy::prelude::*;

#[derive(Component)]
pub struct Animator {
    animations: StateGraph<Animation>,
    frame_iterator: TextureIterator,
    last_frame_time: Duration,
}

//impl Animator {
//    pub fn new(
