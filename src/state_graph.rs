use crate::variable_manager::{Condition, VariableManager};
use std::collections::HashMap;

/// A state transition that holds a target state and a vector of conditions
/// that must be met to trigger the transition.
#[derive(Clone)]
pub struct Transition {
    /// The target state that the transition leads to.
    pub target: String,
    /// A vector of conditions that must be met for the transition to occur.
    pub conditions: Vec<Condition>,
}

/// Represents a node in a StateGraph.  
#[derive(Clone)]
pub struct GraphNode<T> {
    /// The data that the node holds.
    pub data: T,
    /// A list of transitions that will be executed in priority of low to high.
    pub transitions: Vec<Transition>,
}

/// A state graph that manages state transitions when conditions are met.
pub struct StateGraph<T> {
    variables: VariableManager,
    nodes: HashMap<String, GraphNode<T>>,
    active: String,
}

impl<T> StateGraph<T> {
    /// Constructs a new `StateGraph` with the specified `default_node_name` and `nodes`.
    ///
    /// # Arguments
    ///
    /// * `default_node_name`: A `String` representing the name of the default node in the graph.
    /// * `nodes`: A `HashMap` mapping node names to `Node<T>` instances.
    ///
    /// # Panics
    ///
    /// Panics if `nodes` does not contain the default node specified by `default_node_name`.
    ///
    pub fn new(default_node_name: String, nodes: HashMap<String, GraphNode<T>>) -> Self {
        assert!(
            nodes.contains_key(&default_node_name),
            "default node not in node map"
        );
        StateGraph {
            variables: VariableManager::default(),
            nodes,
            active: default_node_name,
        }
    }

    /// Gets the active node's name and data.
    ///
    /// # Returns
    ///
    /// A tuple with two arguments. The first is the name of the active
    /// node. The second is the data that is stored inside the active node.
    ///
    /// # Panics
    ///
    /// This function will panic if the active node does not exist.
    ///
    pub fn get_active(&self) -> (&String, &T) {
        (
            &self.active,
            &self
                .nodes
                .get(&self.active)
                .expect("active node not found")
                .data,
        )
    }

    /// Sets the active node without taking into account transitions.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the node that will be set to active.
    ///
    /// # Panics
    ///
    /// This function will panic if the next active node does not exist
    ///
    pub fn set_active(&mut self, name: String) {
        if !self.nodes.contains_key(&name) {
            panic!("node '{}' does not exist", name);
        }
        self.active = name;
    }

    /// Inserts a node to the state graph with the given name and data.
    ///
    /// # Arguments
    ///
    /// * `name` - The unique name for the new node.
    /// * `data` - The data associated with the new node.
    ///
    pub fn set_node(&mut self, name: String, node: GraphNode<T>) {
        self.nodes.insert(name, node);
    }

    /// Sets the value of a float variable used in the state transitions.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to set.
    /// * `value` - The value to set the variable to.
    ///
    pub fn set_float(&mut self, name: String, value: f32) {
        self.variables.set_float(name, value);
    }

    /// Sets a trigger value to true.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to set.
    ///
    pub fn set_trigger(&mut self, name: String) {
        self.variables.set_trigger(name);
    }
    
    /// Resets all the triggers back to false.
    ///
    pub fn reset_triggers(&mut self) {
        self.variables.reset_triggers();
    }

    /// Transitions through the states of the state graph until a halting state is reached.
    ///
    /// # Panics
    ///
    /// If the function transitions recursively more times than the number of nodes in the graph, a panic occurs.
    ///
    pub fn transition_until_halt(&mut self) -> bool {
        // Triggers are reset after first transition.
        if !self.transition() {
            return false;
        };
        self.variables.reset_triggers();

        for _ in 0..self.nodes.len() {
            if !self.transition() {
                return true;
            };
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
        let active = self
            .nodes
            .get(&self.active)
            .unwrap_or_else(|| panic!("active node '{}' not found", &self.active));
        for transition in active.transitions.iter() {
            if Condition::all_true(&transition.conditions, &self.variables) {
                if !self.nodes.contains_key(&transition.target) {
                    panic!(
                        "tried to transition to non-existant target '{}'",
                        transition.target
                    );
                }
                self.active = transition.target.clone();
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
        let mut nodes: HashMap<String, GraphNode<String>> = HashMap::default();
        let n1 = GraphNode {
            data: "A1".to_string(),
            transitions: vec![Transition {
                target: "N2".to_string(),
                conditions: vec![Condition::Eq("V1".to_string(), 0.0)],
            }],
        };
        nodes.insert("N1".to_string(), n1);
        let n2 = GraphNode {
            data: "A2".to_string(),
            transitions: vec![],
        };
        nodes.insert("N2".to_string(), n2);
        let mut state_graph = StateGraph::new("N1".to_string(), nodes);
        state_graph.set_float("V1".to_string(), 1.0);

        state_graph.transition_until_halt();
        assert_eq!(state_graph.get_active().0.clone(), "N1".to_string());
        state_graph.set_float("V1".to_string(), 0.0);
        state_graph.transition_until_halt();
        assert_eq!(state_graph.get_active().0.clone(), "N2".to_string());
    }

    #[test]
    fn test_chained_transition() {
        let mut nodes: HashMap<String, GraphNode<String>> = HashMap::default();
        let n1 = GraphNode {
            data: "A1".to_string(),
            transitions: vec![Transition {
                target: "N2".to_string(),
                conditions: vec![Condition::Eq("V1".to_string(), 0.0)],
            }],
        };
        nodes.insert("N1".to_string(), n1);
        let n2 = GraphNode {
            data: "A2".to_string(),
            transitions: vec![Transition {
                target: "N3".to_string(),
                conditions: vec![Condition::Eq("V1".to_string(), 0.0)],
            }],
        };
        nodes.insert("N2".to_string(), n2);
        let n3 = GraphNode {
            data: "A3".to_string(),
            transitions: vec![],
        };
        nodes.insert("N3".to_string(), n3);
        let mut state_graph = StateGraph::new("N1".to_string(), nodes);
        state_graph.set_float("V1".to_string(), 0.0);

        state_graph.transition_until_halt();
        assert_eq!(state_graph.get_active().0.clone(), "N3".to_string());
    }

    #[test]
    fn test_trigger_transitions() {
        let mut nodes: HashMap<String, GraphNode<String>> = HashMap::default();
        let n1 = GraphNode {
            data: "A1".to_string(),
            transitions: vec![Transition {
                target: "N2".to_string(),
                conditions: vec![Condition::Trigger("T1".to_string())],
            }],
        };
        let n2 = GraphNode {
            data: "A2".to_string(),
            transitions: vec![],
        };
        nodes.insert("N1".to_string(), n1);
        nodes.insert("N2".to_string(), n2);
        let mut state_graph = StateGraph::new("N1".to_string(), nodes);

        state_graph.transition_until_halt();
        assert_eq!(state_graph.get_active().0.clone(), "N1".to_string());
        state_graph.set_trigger("T1".to_string());
        state_graph.transition_until_halt();
        assert_eq!(state_graph.get_active().0.clone(), "N2".to_string());
    }
}
