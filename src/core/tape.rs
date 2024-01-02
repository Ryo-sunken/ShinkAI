use crate::core::handle::VariableHandle;
use matrix::matrix::{Axis, Matrix};
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Function {
    Add(usize, usize),
    Sub(usize, usize),
    MatMul(usize, usize),
    Neg(usize),
    ScalarMul(usize, usize),
    ScalarDiv(usize, usize),
    CWiseMul(usize, usize),
    CWiseDiv(usize, usize),
    Transpose(usize),
    Pow(usize, usize),
    Exp(usize),
    Sin(usize),
    Cos(usize),
    Sqrt(usize),
    Min(usize, Option<Axis>),
    Max(usize, Option<Axis>),
    Sum(usize, Option<Axis>),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Variable<T> {
    pub(crate) data: Matrix<T>,
    pub(crate) grad: Option<Matrix<T>>,
    pub(crate) creator: Option<Function>,
    pub(crate) generation: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct VariableTape<T> {
    pub(crate) nodes: RefCell<Vec<Variable<T>>>,
}

fn max(x: usize, y: usize) -> usize {
    if x > y {
        x
    } else {
        y
    }
}

impl<T> VariableTape<T> {
    pub fn new() -> Self {
        Self {
            nodes: RefCell::new(Vec::new()),
        }
    }

    pub fn variable(&self, data: Matrix<T>) -> VariableHandle<T> {
        self.nodes.borrow_mut().push(Variable {
            data,
            grad: None,
            creator: None,
            generation: 0,
        });
        VariableHandle::<T>::new(self, self.nodes.borrow().len() - 1)
    }

    pub(crate) fn from_function(&self, data: Matrix<T>, creator: Function) -> VariableHandle<T> {
        let generation = self.generation(creator);
        self.nodes.borrow_mut().push(Variable {
            data,
            grad: None,
            creator: Some(creator),
            generation,
        });
        VariableHandle::<T>::new(self, self.nodes.borrow().len() - 1)
    }

    fn generation(&self, creator: Function) -> usize {
        // TODO: Functionごとのgenerationの計算
        1
    }
}
