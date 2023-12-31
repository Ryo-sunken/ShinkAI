use crate::core::{handle::VariableHandle, tape::Function};
use matrix::matrix::Matrix;
use num_traits::Float;
use std::ops::{Div, Mul, Neg};

macro_rules! defscalarmul {
    ( $( $t: ty ),+ ) => {
        $(
            impl<'a> Mul<VariableHandle<'a, $t>> for $t {
                type Output = VariableHandle<'a, $t>;

                fn mul(self, rhs: VariableHandle<'a, $t>) -> Self::Output {
                    let self_handle = rhs.tape.variable(Matrix::new([[self]]));
                    let result = {
                        let binding = rhs.tape.nodes.borrow();
                        let var_rhs = &binding.get(rhs.data_idx).unwrap().data;
                        self * var_rhs
                    };
                    rhs.tape
                        .from_function(result, Function::ScalarMul(rhs.data_idx, self_handle.data_idx))
                }
            }
        )+
    };
}

defscalarmul![f32, f64];

impl<'a, T: Float> Neg for VariableHandle<'a, T> {
    type Output = VariableHandle<'a, T>;

    fn neg(self) -> Self::Output {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            -var_self
        };
        self.tape
            .from_function(result, Function::Neg(self.data_idx))
    }
}

impl<'a, T: Float> Mul<T> for VariableHandle<'a, T> {
    type Output = VariableHandle<'a, T>;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs_handle = self.tape.variable(Matrix::new([[rhs]]));
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            var_self * rhs
        };
        self.tape.from_function(
            result,
            Function::ScalarMul(self.data_idx, rhs_handle.data_idx),
        )
    }
}

impl<'a, T: Float> Div<T> for VariableHandle<'a, T> {
    type Output = VariableHandle<'a, T>;

    fn div(self, rhs: T) -> Self::Output {
        let rhs_handle = self.tape.variable(Matrix::new([[rhs]]));
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = &binding.get(self.data_idx).unwrap().data;
            var_self / rhs
        };
        self.tape.from_function(
            result,
            Function::ScalarDiv(self.data_idx, rhs_handle.data_idx),
        )
    }
}
