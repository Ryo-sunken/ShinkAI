use crate::core::handle::{Function, VariableHandle};
use std::ops::{Div, Mul, Neg};

macro_rules! defscalarmul {
    ( $( $t: ty ),+ ) => {
        $(
            impl<'a> Mul<VariableHandle<'a, $t>> for $t {
                type Output = VariableHandle<'a, $t>;

                fn mul(self, rhs: VariableHandle<'a, $t>) -> Self::Output {
                    let result = {
                        let binding = rhs.tape.nodes.borrow();
                        let var_rhs = binding.get(rhs.data_idx).unwrap();
                        self * var_rhs
                    };
                    rhs.tape
                        .variable(result)
                        .set_creator(Function::ScalarMul(rhs.data_idx, self))
                }
            }
        )+
    };
}

defscalarmul![i8, i16, i32, isize, u8, u16, u32, usize, f32, f64];

impl<'a, T> Neg for VariableHandle<'a, T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = VariableHandle<'a, T>;

    fn neg(self) -> Self::Output {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = binding.get(self.data_idx).unwrap();
            -var_self
        };
        self.tape
            .variable(result)
            .set_creator(Function::Neg(self.data_idx))
    }
}

impl<'a, T> Mul<T> for VariableHandle<'a, T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = VariableHandle<'a, T>;

    fn mul(self, rhs: T) -> Self::Output {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = binding.get(self.data_idx).unwrap();
            var_self * rhs
        };
        self.tape
            .variable(result)
            .set_creator(Function::ScalarMul(self.data_idx, rhs))
    }
}

impl<'a, T> Div<T> for VariableHandle<'a, T>
where
    T: Div<Output = T> + Copy,
{
    type Output = VariableHandle<'a, T>;

    fn div(self, rhs: T) -> Self::Output {
        let result = {
            let binding = self.tape.nodes.borrow();
            let var_self = binding.get(self.data_idx).unwrap();
            var_self / rhs
        };
        self.tape
            .variable(result)
            .set_creator(Function::ScalarDiv(self.data_idx, rhs))
    }
}
