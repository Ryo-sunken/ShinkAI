use crate::core::handle::VariableHandle;
use matrix::matrix::Matrix;
use std::{arch::x86_64, cell::RefCell};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Function<T> {
    Add(usize, usize),
    Sub(usize, usize),
    MatMul(usize, usize),
    Neg(usize),
    ScalarMul(usize, T),
    ScalarDiv(usize, T),
    CWiseMul(usize, usize),
    CWiseDiv(usize, usize),
    Transpose(usize),
    Pow(usize, T),
    Exp(usize),
    Sin(usize),
    Cos(usize),
    Sqrt(usize),
    Min(usize, Option<Axis>),
    Max(usize, Option<Axis>),
    Sum(usize, Option<Axis>),
}

#[allow(dead_code)]
pub struct Variable<T> {
    pub(crate) data: Matrix<T>,
    pub(crate) grad: Option<Matrix<T>>,
    pub(crate) creator: Option<Function<T>>,
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

    pub fn from_function(&self, data: Matrix<T>, creator: Function<T>) -> VariableHandle<T> {
        let generation = self.generation(creator);
        self.nodes.borrow_mut().push(Variable {
            data,
            grad: None,
            creator: Some(creator),
            generation,
        });
        VariableHandle::<T>::new(self, self.nodes.borrow().len() - 1)
    }

    fn generation(&self, creator: Function<T>) -> usize {
        // TODO: Functionごとのgenerationの計算
        1
    }
}
