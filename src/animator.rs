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
    pub play_next: Option<String>,
}

impl Animator {
    pub fn new(state_graph: StateGraph<AnimationData>) -> Self {
        Animator {
            state_graph,
            frame_iterator: None,
            last_frame_time: None,
            play_next: None,
        }
    }

    pub fn transition_to(&mut self, name: String) {
        let (active_name, _) = self.state_graph.get_active();
        if *active_name == name {
            return;
        }
        self.play_next = Some(name);
    }

    fn sync_frame_iterator(&mut self) {
        let (_, active_animation) = self.state_graph.get_active();
        let (first_frame, last_frame) = (
            active_animation.animation.first_frame,
            active_animation.animation.last_frame,
        );
        self.frame_iterator = Some(FrameIterator::new(first_frame, last_frame));
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate);
    }
}

fn animate(
    mut query: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut TextureAtlasSprite,
            &mut Animator,
        ),
        With<Animator>,
    >,
    time: Res<Time>,
) {
    for (mut texture_atlas, mut sprite, mut animator) in query.iter_mut() {
        // Initialize frame iterator and last frame time if they are None.

        // If no frame exit time.
        //      Play next if exists.
        //      Transition, if successful reset iterator
        //      update texture, and sprite

        // If frame exit time
        //      If no next, transition
        //      If last frame and play next, play next, reset iterator, update texture and sprite
        //      If last frame, sync iterator, texture, and sprite with current transition
        //      Else only update sprite

        // Go to next frame
        // Continue

        if animator.frame_iterator.is_none() {
            animator.sync_frame_iterator();
        }
        if animator.last_frame_time.is_none() {
            animator.last_frame_time = Some(time.elapsed());
        }

        let animation_data = animator.state_graph.get_active().1;
        if !animation_data.has_exit_time {
            if let Some(next) = animator.play_next.take() {
                animator.state_graph.set_active(next);
            }
            if animator.state_graph.transition_until_halt() {
                animator.sync_frame_iterator();
            }
            let animation_data = animator.state_graph.get_active().1;
            let frame_iterator = animator.frame_iterator.as_ref().unwrap();
            *texture_atlas = animation_data.animation.texture.clone();
            *sprite = TextureAtlasSprite::new(frame_iterator.current());
        } else {
            if animator.play_next.is_none() {
                animator.state_graph.transition_until_halt();
            }
            let frame_iterator = animator.frame_iterator.as_ref().unwrap();
            if frame_iterator.is_last_frame() {
                if let Some(next) = animator.play_next.take() {
                    animator.state_graph.set_active(next);
                }
                animator.sync_frame_iterator();
                let animation_data = animator.state_graph.get_active().1;
                let frame_iterator = animator.frame_iterator.as_ref().unwrap();
                *texture_atlas = animation_data.animation.texture.clone();
                *sprite = TextureAtlasSprite::new(frame_iterator.current());
            } else {
                let frame_iterator = animator.frame_iterator.as_ref().unwrap();
                *sprite = TextureAtlasSprite::new(frame_iterator.current());
            }
        }

        let frame_iterator = animator.frame_iterator.as_mut().unwrap();
        frame_iterator.next();
    }
}
