use crate::core::tape::VariableTape;
use matrix::matrix::{Axis, Matrix};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct VariableHandle<'a, T> {
    pub(crate) tape: &'a VariableTape<T>,
    pub(crate) data_idx: usize,
}

impl<'a, T> VariableHandle<'a, T> {
    pub(crate) fn new(tape: &'a VariableTape<T>, data_idx: usize) -> Self {
        Self { tape, data_idx }
    }
}

impl<'a, T> VariableHandle<'a, T>
where
    T: Clone,
{
    pub fn data(self) -> Matrix<T> {
        self.tape
            .nodes
            .borrow()
            .get(self.data_idx)
            .cloned()
            .unwrap()
    }
}
