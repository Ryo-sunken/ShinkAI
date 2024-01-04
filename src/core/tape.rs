use crate::core::handle::VariableHandle;
use matrix::matrix::{Axis, Matrix};
use num_traits::Float;
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Function {
    Add(usize, usize),
    Sub(usize, usize),
    MatMul(usize, usize),
    ScalarMul(usize, usize),
    ScalarDiv(usize, usize),
    CWiseMul(usize, usize),
    CWiseDiv(usize, usize),
    CWiseMax(usize, usize),
    CWiseMin(usize, usize),
    Pow(usize, usize),
    Transpose(usize),
    Neg(usize),
    Exp(usize),
    Sin(usize),
    Cos(usize),
    Sqrt(usize),
    Concat(usize, Axis),
    Max(usize, Option<Axis>),
    Min(usize, Option<Axis>),
    Sum(usize, Option<Axis>),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Variable<T: Float> {
    pub(crate) data: Matrix<T>,
    pub(crate) grad: Option<Matrix<T>>,
    pub(crate) creator: Option<Function>,
    pub(crate) generation: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct VariableTape<T: Float> {
    pub(crate) nodes: RefCell<Vec<Variable<T>>>,
}

fn max(x: usize, y: usize) -> usize {
    if x > y {
        x
    } else {
        y
    }
}

impl<T: Float> VariableTape<T> {
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
        match creator {
            // 二項演算
            Function::Add(x_idx, y_idx)
            | Function::Sub(x_idx, y_idx)
            | Function::MatMul(x_idx, y_idx)
            | Function::ScalarMul(x_idx, y_idx)
            | Function::ScalarDiv(x_idx, y_idx)
            | Function::CWiseMul(x_idx, y_idx)
            | Function::CWiseDiv(x_idx, y_idx)
            | Function::CWiseMax(x_idx, y_idx)
            | Function::CWiseMin(x_idx, y_idx)
            | Function::Pow(x_idx, y_idx) => {
                let binding = self.nodes.borrow();
                let x_gen = binding.get(x_idx).unwrap().generation;
                let y_gen = binding.get(y_idx).unwrap().generation;
                max(x_gen, y_gen) + 1
            },
            // 一変数関数
            Function::Transpose(idx)
            | Function::Neg(idx)
            | Function::Exp(idx)
            | Function::Sin(idx)
            | Function::Cos(idx)
            | Function::Sqrt(idx)
            | Function::Concat(idx, _)
            | Function::Max(idx, _)
            | Function::Min(idx, _)
            | Function::Sum(idx, _) => {
                let binding = self.nodes.borrow();
                let gen = binding.get(idx).unwrap().generation;
                gen + 1
            },
        }
    }
}
