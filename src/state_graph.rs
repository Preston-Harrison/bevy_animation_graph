use std::collections::HashMap;

macro_rules! unwrap_or_return {
    ( $e:expr, $v:expr ) => {
        match $e {
            Some(x) => x,
            None => return $v,
        }
    };
}

#[derive(Clone)]
pub enum Condition {
    Gt(String, String),
    Lt(String, String),
    Eq(String, String),
}

impl Condition {
    fn eval(&self, variables: &HashMap<String, f32>) -> bool {
        match self {
            Condition::Gt(a, b) => match (variables.get(a), variables.get(b)) {
                (Some(v1), Some(v2)) => v1 > v2,
                _ => false,
            },
            Condition::Lt(a, b) => match (variables.get(a), variables.get(b)) {
                (Some(v1), Some(v2)) => v1 < v2,
                _ => false,
            },
            Condition::Eq(a, b) => match (variables.get(a), variables.get(b)) {
                (Some(v1), Some(v2)) => v1 == v2,
                _ => false,
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
    true
}

#[derive(Clone)]
pub struct Transition {
    to: String,
    conditions: Vec<Condition>,
}

#[derive(Clone)]
pub struct Node<T> {
    state: T,
    transitions: Vec<Transition>,
    has_exit_time: bool,
}

#[derive(Default)]
pub struct StateGraph<T> {
    variables: HashMap<String, f32>,
    pub nodes: HashMap<String, Node<T>>,
    pub active: Option<String>,
}

impl<T> StateGraph<T> {
    pub fn set_variable(&mut self, name: String, value: f32) {
        self.variables.insert(name, value);
    }

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
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_transition() {
        let mut state_graph = StateGraph::default();
        state_graph.set_variable("V1".to_string(), 0.0);
        state_graph.set_variable("V2".to_string(), 1.0);
        let n1 = Node {
            state: "A1".to_string(),
            transitions: vec![Transition {
                to: "N2".to_string(),
                conditions: vec![Condition::Eq("V1".to_string(), "V2".to_string())],
            }],
            has_exit_time: false,
        };
        state_graph.nodes.insert("N1".to_string(), n1);
        let n2 = Node {
            state: "A2".to_string(),
            transitions: vec![],
            has_exit_time: false,
        };
        state_graph.nodes.insert("N2".to_string(), n2);

        state_graph.active = Some("N1".to_string());
        state_graph.recurse_transition(false);
        assert_eq!(state_graph.active, Some("N1".to_string()));
        state_graph.set_variable("V2".to_string(), 0.0);
        state_graph.recurse_transition(false);
        assert_eq!(state_graph.active, Some("N2".to_string()));
    }

    #[test]
    fn test_complicated_transitions() {
        let mut state_graph = StateGraph::default();
        state_graph.set_variable("V1".to_string(), 0.0);
        state_graph.set_variable("V2".to_string(), 0.0);
        let n1 = Node {
            state: "A1".to_string(),
            transitions: vec![Transition {
                to: "N2".to_string(),
                conditions: vec![Condition::Eq("V1".to_string(), "V2".to_string())],
            }],
            has_exit_time: false,
        };
        state_graph.nodes.insert("N1".to_string(), n1);
        let n2 = Node {
            state: "A2".to_string(),
            transitions: vec![Transition {
                to: "N3".to_string(),
                conditions: vec![Condition::Eq("V1".to_string(), "V2".to_string())],
            }],
            has_exit_time: false,
        };
        state_graph.nodes.insert("N2".to_string(), n2);
        let n3 = Node {
            state: "A2".to_string(),
            transitions: vec![],
            has_exit_time: false,
        };
        state_graph.nodes.insert("N3".to_string(), n3);
        state_graph.active = Some("N1".to_string());
        state_graph.recurse_transition(false);
        assert_eq!(state_graph.active, Some("N3".to_string()));
    }
}
