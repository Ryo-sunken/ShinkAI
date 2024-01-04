use crate::core::{handle::VariableHandle, tape::Function};
use num_traits::Float;
use std::{iter::Sum, ops::Mul};

impl<'a, T: Float> VariableHandle<'a, T> {
    pub fn cwise_mul(self, rhs: Self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            let var_rhs = &binding.get(rhs.data_idx).unwrap().data;
            var_self.cwise_mul(var_rhs)
        };
        self.tape
            .from_function(result, Function::CWiseMul(self.data_idx, rhs.data_idx))
    }
}

impl<'a, T: Float> VariableHandle<'a, T> {
    pub fn cwise_div(self, rhs: Self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            let var_rhs = &binding.get(rhs.data_idx).unwrap().data;
            var_self.cwise_div(var_rhs)
        };
        self.tape
            .from_function(result, Function::CWiseDiv(self.data_idx, rhs.data_idx))
    }
}

impl<'a, T> Mul for VariableHandle<'a, T>
where
    T: Sum + Float,
{
    type Output = VariableHandle<'a, T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            let var_rhs = &binding.get(rhs.data_idx).unwrap().data;
            var_self * var_rhs
        };
        self.tape
            .from_function(result, Function::MatMul(self.data_idx, rhs.data_idx))
    }
}
