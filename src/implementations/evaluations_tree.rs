use crate::types::Evaluator;
use rand::Rng;

use super::{
    constants::{NODE_VALUE_MAX, NODE_VALUE_MIN, STATE_SIZE},
    state_array::StateArray,
    utils::{self, dot_product},
};

struct EvaluationNode {
    left: Box<dyn Evaluator<StateArray>>,
    right: Box<dyn Evaluator<StateArray>>,
    value: i32,
    index: usize,
}

fn new_random_node(
    left: Box<dyn Evaluator<StateArray>>,
    right: Box<dyn Evaluator<StateArray>>,
) -> EvaluationNode {
    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..STATE_SIZE);
    let value = rng.gen_range(NODE_VALUE_MIN..NODE_VALUE_MAX - 1);

    EvaluationNode {
        left,
        right,
        value,
        index,
    }
}

impl Evaluator<StateArray> for EvaluationNode {
    fn evaluate(&self, state: StateArray) -> i32 {
        if state.0[self.index] <= self.value {
            self.left.evaluate(state)
        } else {
            self.right.evaluate(state)
        }
    }
}

struct EvaluationLeaf([i32; STATE_SIZE]);

fn new_random_leaf() -> EvaluationLeaf {
    EvaluationLeaf(utils::random_array(NODE_VALUE_MIN, NODE_VALUE_MAX))
}

impl Evaluator<StateArray> for EvaluationLeaf {
    fn evaluate(&self, state: StateArray) -> i32 {
        dot_product(self.0, state.0)
    }
}

pub fn build_evaluations_tree(depth: usize) -> Box<dyn Evaluator<StateArray>> {
    if depth == 0 {
        Box::new(new_random_leaf())
    } else {
        let left = build_evaluations_tree(depth - 1);
        let right = build_evaluations_tree(depth - 1);

        Box::new(new_random_node(left, right))
    }
}
