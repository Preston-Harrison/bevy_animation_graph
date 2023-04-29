use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct VariableManager {
    triggers: HashSet<String>,
    floats: HashMap<String, f32>,
}

impl VariableManager {
    pub fn set_float(&mut self, name: String, value: f32) {
        self.floats.insert(name, value);
    }

    pub fn set_trigger(&mut self, name: String) {
        self.triggers.insert(name);
    }

    pub fn get_float(&self, name: &String) -> Option<&f32> {
        self.floats.get(name)
    }

    pub fn get_trigger(&self, name: &String) -> bool {
        self.triggers.contains(name)
    }

    pub fn reset_triggers(&mut self) {
        self.triggers.clear();
    }
}

#[derive(Clone)]
pub enum Condition {
    Gt(String, String),
    Lt(String, String),
    Eq(String, String),
    Trigger(String),
}

impl Condition {
    fn eval(&self, variables: &VariableManager) -> bool {
        match self {
            Condition::Gt(a, b) => match (variables.get_float(a), variables.get_float(b)) {
                (Some(v1), Some(v2)) => v1 > v2,
                _ => false,
            },
            Condition::Lt(a, b) => match (variables.get_float(a), variables.get_float(b)) {
                (Some(v1), Some(v2)) => v1 < v2,
                _ => false,
            },
            Condition::Eq(a, b) => match (variables.get_float(a), variables.get_float(b)) {
                (Some(v1), Some(v2)) => v1 == v2,
                _ => false,
            },
            Condition::Trigger(a) => variables.get_trigger(a),
        }
    }

    pub fn all_true(conditions: &[Condition], variables: &VariableManager) -> bool {
        for condition in conditions {
            if !condition.eval(variables) {
                return false;
            }
        }
        true
    }
}
