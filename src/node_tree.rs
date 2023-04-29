use std::collections::HashMap;
use crate::animation::Animation;

macro_rules! unwrap_or_return {
    ( $e:expr, $v:expr ) => {
        match $e {
            Some(x) => x,
            None => return $v,
        }
    }
}

pub enum Condition {
    Gt(String, String),
    Lt(String, String),
    Eq(String, String),
}

impl Condition {
    fn eval(&self, variables: &HashMap<String, f32>) -> bool {
        match self {
            Condition::Gt(a, b) => {
                match (variables.get(a), variables.get(b)) {
                    (Some(v1), Some(v2)) => v1 > v2,
                    _ => false
                }
            },
            Condition::Lt(a, b) => {
                match (variables.get(a), variables.get(b)) {
                    (Some(v1), Some(v2)) => v1 < v2,
                    _ => false
                }
            },
            Condition::Eq(a, b) => {
                match (variables.get(a), variables.get(b)) {
                    (Some(v1), Some(v2)) => v1 == v2,
                    _ => false
                }
            },
        }
    }
}

fn and_conditions(conditions: &[Condition], variables: &HashMap<String, f32>) -> bool {
    for condition in conditions {
        if !condition.eval(variables) {
            return false;
        }
    }
    return true;
}

pub struct Transition {
    to: String,
    conditions: Vec<Condition>
}

pub struct Node {
    animation: Animation,
    transitions: Vec<Transition>, 
    has_exit_time: bool,
}

#[derive(Default)]
pub struct NodeTree {
    pub variables: HashMap<String, f32>,
    pub nodes: HashMap<String, Node>,
    active: Option<String>,
}

impl NodeTree {
    fn recurse_transition(&mut self, is_last_frame: bool) {
        if !self.transition(is_last_frame) {
            return;
        }
        let mut transitions = 1;
        while transitions < self.nodes.len() {
            if !self.transition(is_last_frame) {
                return;
            }
            transitions += 1;
        }
        panic!("recursive transition loop");
    }

    fn transition(&mut self, is_last_frame: bool) -> bool {
        let active_name = unwrap_or_return!(&self.active, false); 
        let active = unwrap_or_return!(self.nodes.get(active_name), false);
        if !is_last_frame && active.has_exit_time {
            return false;
        }
        for transition in active.transitions.iter() {
            let should_transition = and_conditions(&transition.conditions, &self.variables);
            if should_transition {
                self.active = Some(transition.to.clone());
                return true;
            }
        }
        return false;
    }
}
