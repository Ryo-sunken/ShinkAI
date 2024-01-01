use std::ops::BitAndAssign;

use crate::core::handle::{Function, VariableHandle};
use num_traits::Float;

impl<'a, T> VariableHandle<'a, T>
where
    T: Float,
{
    pub fn exp(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = binding.get(self.data_idx).unwrap();
            var_self.exp()
        };
        self.tape
            .variable(result)
            .set_creator(Function::Exp(self.data_idx))
    }

    pub fn sin(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = binding.get(self.data_idx).unwrap();
            var_self.sin()
        };
        self.tape
            .variable(result)
            .set_creator(Function::Sin(self.data_idx))
    }

    pub fn cos(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = binding.get(self.data_idx).unwrap();
            var_self.cos()
        };
        self.tape
            .variable(result)
            .set_creator(Function::Cos(self.data_idx))
    }

    pub fn sqrt(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = binding.get(self.data_idx).unwrap();
            var_self.sqrt()
        };
        self.tape
            .variable(result)
            .set_creator(Function::Sqrt(self.data_idx))
    }
}
