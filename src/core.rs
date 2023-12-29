use std::cell::RefCell;
use matrix::matrix::Matrix;
use crate::variable::*;

#[derive(Clone, Debug)]
pub struct VariableTape<T>
{
    nodes: RefCell<Vec<Variable<T>>>,
}

#[derive(Clone, Copy, Debug)]
pub struct VariableHandle<'a, T>
{
    tape: &'a VariableTape<T>,
    idx: Index,
}

impl<T> VariableTape<T>
{
    pub fn new() -> Self
    {
        Self
        {
            nodes: RefCell::new(Vec::new()),
        }
    }

    pub fn variable(&self, data: Matrix<T>) -> VariableHandle<T>
    {
        self.nodes.borrow_mut().push(Variable::<T>::new(data));
        VariableHandle::<T>::new(self, self.nodes.borrow().len())
    }

    pub fn from_function(&self, data: Matrix<T>, creator: Function<T>) -> VariableHandle<T>
    {
        self.nodes.borrow_mut().push(Variable::<T>::from_function(data, creator));
        VariableHandle::<T>::new(self, self.nodes.borrow().len())
    }
}

impl<'a, T> VariableHandle<'a, T>
{
    fn new(tape: &'a VariableTape<T>, idx: Index) -> Self
    {
        Self { tape, idx }
    }
}

// TODO VariableHandleの演算