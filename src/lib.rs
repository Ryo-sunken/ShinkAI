pub mod core;
mod functions;

#[cfg(test)]
mod tests {
    use crate::core::{handle::Function, tape::VariableTape};
    use matrix::matrix::Matrix;

    #[test]
    fn add() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[1., 2., 3.]]));
        let y = tape.variable(Matrix::new([[4., 5., 6.]]));
        let z = x + y;
        assert_eq!(z.data(), Matrix::new([[5., 7., 9.]]));
        assert_eq!(z.creator.unwrap(), Function::Add(x.data_idx, y.data_idx));
    }

    #[test]
    fn sub() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[4., 5., 6.]]));
        let y = tape.variable(Matrix::new([[1., 2., 3.]]));
        let z = x - y;
        assert_eq!(z.data(), Matrix::new([[3., 3., 3.]]));
        assert_eq!(z.creator.unwrap(), Function::Sub(x.data_idx, y.data_idx));
    }

    #[test]
    fn cwise_mul() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[1.,2.,3.]]));
        let y = tape.variable(Matrix::new([[2.,3.,4.]]));
        let z = x.cwise_mul(y);
        assert_eq!(z.data(), Matrix::new([[2.,6.,12.]]));
        assert_eq!(z.creator.unwrap(), Function::CWiseMul(x.data_idx, y.data_idx));
    }

    #[test]
    fn cwise_div() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[6.,9.,10.]]));
        let y = tape.variable(Matrix::new([[2.,3.,5.]]));
        let z = x.cwise_div(y);
        assert_eq!(z.data(), Matrix::new([[3.,3.,2.]]));
        assert_eq!(z.creator.unwrap(), Function::CWiseDiv(x.data_idx, y.data_idx));
    }
}
