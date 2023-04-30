mod animator;
mod frame_iterator;
mod state_graph;
mod variable_manager;

pub use animator::{Animator, Animation, AnimationData, AnimationPlugin};
pub use state_graph::{Node, StateGraph, Transition};
pub use variable_manager::Condition;
