use std::collections::HashMap;

// TODO either move to utils or get rid of this
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

    fn all_true(conditions: &[Condition], variables: &HashMap<String, f32>) -> bool {
        for condition in conditions {
            if !condition.eval(variables) {
                return false;
            }
        }
        true
    }
}

/// A state transition that holds a target state and a vector of conditions 
/// that must be met to trigger the transition.
#[derive(Clone)]
pub struct Transition {
    /// The target state that the transition leads to.
    target: String,
    /// A vector of conditions that must be met for the transition to occur.
    conditions: Vec<Condition>,
}

/// Represents a node in a StateGraph.  
#[derive(Clone)]
pub struct Node<T> {
    /// The data that the node holds.
    data: T,
    /// A list of transitions that will be executed in priority of low to high.
    transitions: Vec<Transition>,
}

/// A state graph that manages state transitions when conditions are met.
#[derive(Default)]
pub struct StateGraph<T> {
    variables: HashMap<String, f32>,
    pub nodes: HashMap<String, Node<T>>,
    pub active: Option<String>,
}

impl<T> StateGraph<T> {
    /// Sets the value of a variable used in the state transitions.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to set.
    /// * `value` - The value to set the variable to.
    ///
    pub fn set_variable(&mut self, name: String, value: f32) {
        self.variables.insert(name, value);
    }

    /// Transitions through the states of the state graph until a halting state is reached.
    ///
    /// # Panics
    ///
    /// If the function transitions recursively more times than the number of nodes in the graph, a panic occurs.
    ///
    pub fn transition_until_halt(&mut self) {
        let mut transitions = 0;
        while transitions < self.nodes.len() {
            if !self.transition() {
                return;
            }
            transitions += 1;
        }
        panic!("recursive transition loop");
    }

    /// Checks all the conditions in the active state and transitions once if any of the
    /// transitions has all of it's conditions met.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether a transition was made.
    ///
    fn transition(&mut self) -> bool {
        let active_name = unwrap_or_return!(&self.active, false);
        let active = unwrap_or_return!(self.nodes.get(active_name), false);
        for transition in active.transitions.iter() {
            let should_transition = Condition::all_true(&transition.conditions, &self.variables);
            if should_transition {
                self.active = Some(transition.target.clone());
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
            data: "A1".to_string(),
            transitions: vec![Transition {
                target: "N2".to_string(),
                conditions: vec![Condition::Eq("V1".to_string(), "V2".to_string())],
            }],
        };
        state_graph.nodes.insert("N1".to_string(), n1);
        let n2 = Node {
            data: "A2".to_string(),
            transitions: vec![],
        };
        state_graph.nodes.insert("N2".to_string(), n2);

        state_graph.active = Some("N1".to_string());
        state_graph.transition_until_halt();
        assert_eq!(state_graph.active, Some("N1".to_string()));
        state_graph.set_variable("V2".to_string(), 0.0);
        state_graph.transition_until_halt();
        assert_eq!(state_graph.active, Some("N2".to_string()));
    }

    #[test]
    fn test_complicated_transitions() {
        let mut state_graph = StateGraph::default();
        state_graph.set_variable("V1".to_string(), 0.0);
        state_graph.set_variable("V2".to_string(), 0.0);
        let n1 = Node {
            data: "A1".to_string(),
            transitions: vec![Transition {
                target: "N2".to_string(),
                conditions: vec![Condition::Eq("V1".to_string(), "V2".to_string())],
            }],
        };
        state_graph.nodes.insert("N1".to_string(), n1);
        let n2 = Node {
            data: "A2".to_string(),
            transitions: vec![Transition {
                target: "N3".to_string(),
                conditions: vec![Condition::Eq("V1".to_string(), "V2".to_string())],
            }],
        };
        state_graph.nodes.insert("N2".to_string(), n2);
        let n3 = Node {
            data: "A2".to_string(),
            transitions: vec![],
        };
        state_graph.nodes.insert("N3".to_string(), n3);
        state_graph.active = Some("N1".to_string());
        state_graph.transition_until_halt();
        assert_eq!(state_graph.active, Some("N3".to_string()));
    }
}
