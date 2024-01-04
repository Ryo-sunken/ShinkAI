use crate::core::{handle::VariableHandle, tape::Function};
use num_traits::Float;
use std::ops::Sub;

impl<'a, T: Float> Sub for VariableHandle<'a, T> {
    type Output = VariableHandle<'a, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            let var_rhs = &binding.get(rhs.data_idx).unwrap().data;
            var_self - var_rhs
        };
        self.tape
            .from_function(result, Function::Sub(self.data_idx, rhs.data_idx))
    }
}
