use crate::core::{handle::VariableHandle, tape::Function};
use num_traits::Float;

impl<'a, T: Float> VariableHandle<'a, T> {

    pub fn exp(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            var_self.exp()
        };
        self.tape
            .from_function(result, Function::Exp(self.data_idx))
    }

    pub fn sin(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            var_self.sin()
        };
        self.tape
            .from_function(result, Function::Sin(self.data_idx))
    }

    pub fn cos(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            var_self.cos()
        };
        self.tape
            .from_function(result, Function::Cos(self.data_idx))
    }

    pub fn sqrt(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            var_self.sqrt()
        };
        self.tape
            .from_function(result, Function::Sqrt(self.data_idx))
    }

    pub fn tanh(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            var_self.tanh()
        };
        self.tape
            .from_function(result, Function::Tanh(self.data_idx))
    }

    pub fn sigmoid(self) -> Self {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            var_self.sigmoid()
        };
        self.tape
            .from_function(result, Function::Sigmoid(self.data_idx))
    }
}
