use crate::{animation::Animation, state_graph::StateGraph, texture_iterator::TextureIterator};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Animator {
    animations: StateGraph<Animation>,
    frame_iterator: TextureIterator,
    last_frame_time: Duration,
}

//impl Animator {
//    pub fn new(
