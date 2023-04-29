//use bevy::prelude::*;
//use std::{collections::HashMap, time::Duration};
//
//use crate::{animation::Animation, texture_iterator::TextureIterator};
//
//#[derive(Component)]
//pub struct Animator {
//    animations: HashMap<String, Animation>,
//    frame_start: Option<Duration>,
//}
//
//// TODO remove some of the panics here
//impl Animator {
//    pub fn new() -> Self {
//        Animator {
//            animations: HashMap::default(),
//            frame_start: None,
//        }
//    }
//
//    pub fn add_animation(&mut self, name: String, animation: Animation) {
//        assert!(!self.animations.contains_key(&name));
//        self.animations.insert(name, animation);
//    }
//
//    pub fn play(&mut self, name: String) {
//        // Return early if the animation is already playing.
//        if self.active == name {
//            return;
//        }
//
//        let animation = self.animations.get(&name).expect("animation not found");
//        self.frame = TextureIterator::new(animation.first(), animation.last());
//        self.active = name;
//        self.frame_start = None;
//    }
//}
