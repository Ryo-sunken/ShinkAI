use crate::core::handle::{Function, VariableHandle};
use std::ops::{Div, Mul};

impl<'a, T> VariableHandle<'a, T>
where
    T: Mul<Output = T> + Copy,
{
    pub fn cwise_mul(self, rhs: Self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = binding.get(self.data_idx).unwrap();
            let var_rhs = binding.get(rhs.data_idx).unwrap();
            var_self.cwise_mul(var_rhs)
        };
        self.tape
            .variable(result)
            .set_creator(Function::CWiseMul(self.data_idx, rhs.data_idx))
    }
}

impl<'a, T> VariableHandle<'a, T>
where
    T: Div<Output = T> + Copy,
{
    pub fn cwise_div(self, rhs: Self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = binding.get(self.data_idx).unwrap();
            let var_rhs = binding.get(rhs.data_idx).unwrap();
            var_self.cwise_div(var_rhs)
        };
        self.tape
            .variable(result)
            .set_creator(Function::CWiseDiv(self.data_idx, rhs.data_idx))
    }
}
