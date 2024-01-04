use crate::core::tape::{Function, VariableTape};
use matrix::matrix::Matrix;
use num_traits::Float;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct VariableHandle<'a, T: Float> {
    pub(crate) tape: &'a VariableTape<T>,
    pub(crate) data_idx: usize,
}

impl<'a, T: Float> VariableHandle<'a, T> {
    pub(crate) fn new(tape: &'a VariableTape<T>, data_idx: usize) -> Self {
        Self { tape, data_idx }
    }
}

impl<'a, T> VariableHandle<'a, T>
where
    T: Float + Clone,
{
    pub fn data(self) -> Matrix<T> {
        self.tape
            .nodes
            .borrow()
            .get(self.data_idx)
            .cloned()
            .unwrap()
            .data
    }

    pub(crate) fn creator(self) -> Option<Function> {
        self.tape
            .nodes
            .borrow()
            .get(self.data_idx)
            .cloned()
            .unwrap()
            .creator
    }

    pub(crate) fn generation(self) -> usize {
        self.tape
            .nodes
            .borrow()
            .get(self.data_idx)
            .cloned()
            .unwrap()
            .generation
    }
}
