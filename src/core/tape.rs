use crate::core::handle::VariableHandle;
use matrix::matrix::Matrix;
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct VariableTape<T> {
    pub(crate) nodes: RefCell<Vec<Matrix<T>>>,
}

impl<T> VariableTape<T> {
    pub fn new() -> Self {
        Self {
            nodes: RefCell::new(Vec::new()),
        }
    }

    pub fn variable(&self, data: Matrix<T>) -> VariableHandle<T> {
        self.nodes.borrow_mut().push(data);
        VariableHandle::<T>::new(self, self.nodes.borrow().len() - 1)
    }
}
