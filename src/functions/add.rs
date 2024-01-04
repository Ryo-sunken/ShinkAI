use crate::core::{handle::VariableHandle, tape::Function};
use num_traits::Float;
use std::ops::Add;

impl<'a, T: Float> Add for VariableHandle<'a, T> {
    type Output = VariableHandle<'a, T>;

    fn add(self, rhs: Self) -> Self::Output {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            let var_rhs = &binding.get(rhs.data_idx).unwrap().data;
            var_self + var_rhs
        };
        self.tape
            .from_function(result, Function::Add(self.data_idx, rhs.data_idx))
    }
}
