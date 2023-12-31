use crate::core::tape::VariableTape;
use matrix::matrix::{Axis, Matrix};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Function<T> {
    Add(usize, usize),
    Sub(usize, usize),
    MatMul(usize, usize),
    ScalarMul(usize, T),
    CWiseMul(usize, usize),
    CWiseDiv(usize, usize),
    Transpose(usize),
    Pow(usize, T),
    Exp(usize),
    Min(usize, Option<Axis>),
    Max(usize, Option<Axis>),
    Sum(usize, Option<Axis>),
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct VariableHandle<'a, T> {
    pub(crate) tape: &'a VariableTape<T>,
    pub(crate) data_idx: usize,
    pub(crate) grad_idx: Option<usize>,
    pub(crate) creator: Option<Function<T>>,
}

impl<'a, T> VariableHandle<'a, T> {
    pub(crate) fn new(tape: &'a VariableTape<T>, data_idx: usize) -> Self {
        Self {
            tape,
            data_idx,
            grad_idx: None,
            creator: None,
        }
    }

    pub(crate) fn set_creator(self, creator: Function<T>) -> Self {
        Self {
            tape: self.tape,
            data_idx: self.data_idx,
            grad_idx: self.grad_idx,
            creator: Some(creator),
        }
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
