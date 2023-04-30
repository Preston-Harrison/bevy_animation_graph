mod animator;
mod frame_iterator;
mod state_graph;
mod variable_manager;

pub use animator::{Animation, AnimationData, AnimationPlugin, Animator};
pub use state_graph::{GraphNode, StateGraph, Transition};
pub use variable_manager::Condition;
