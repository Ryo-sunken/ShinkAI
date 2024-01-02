pub mod core;
mod functions;

#[cfg(test)]
mod tests {
    use crate::core::{
        handle::VariableHandle,
        tape::{Function, VariableTape},
    };
    use matrix::matrix::Matrix;

    #[test]
    fn add() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[1., 2., 3.]]));
        let y = tape.variable(Matrix::new([[4., 5., 6.]]));
        let z = x + y;
        assert_eq!(z.data(), Matrix::new([[5., 7., 9.]]));
        assert_eq!(z.creator().unwrap(), Function::Add(x.data_idx, y.data_idx));
        assert_eq!(x.generation(), 0);
        assert_eq!(y.generation(), 0);
        assert_eq!(z.generation(), 1);
    }

    #[test]
    fn sub() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[4., 5., 6.]]));
        let y = tape.variable(Matrix::new([[1., 2., 3.]]));
        let z = x - y;
        assert_eq!(z.data(), Matrix::new([[3., 3., 3.]]));
        assert_eq!(z.creator().unwrap(), Function::Sub(x.data_idx, y.data_idx));
        assert_eq!(x.generation(), 0);
        assert_eq!(y.generation(), 0);
        assert_eq!(z.generation(), 1);
    }

    #[test]
    fn cwise_mul() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[1., 2., 3.]]));
        let y = tape.variable(Matrix::new([[2., 3., 4.]]));
        let z = x.cwise_mul(y);
        assert_eq!(z.data(), Matrix::new([[2., 6., 12.]]));
        assert_eq!(
            z.creator().unwrap(),
            Function::CWiseMul(x.data_idx, y.data_idx)
        );
        assert_eq!(x.generation(), 0);
        assert_eq!(y.generation(), 0);
        assert_eq!(z.generation(), 1);
    }

    #[test]
    fn cwise_div() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[6., 9., 10.]]));
        let y = tape.variable(Matrix::new([[2., 3., 5.]]));
        let z = x.cwise_div(y);
        assert_eq!(z.data(), Matrix::new([[3., 3., 2.]]));
        assert_eq!(
            z.creator().unwrap(),
            Function::CWiseDiv(x.data_idx, y.data_idx)
        );
        assert_eq!(x.generation(), 0);
        assert_eq!(y.generation(), 0);
        assert_eq!(z.generation(), 1);
    }

    #[test]
    fn matmul() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[1., 2.], [3., 4.]]));
        let y = tape.variable(Matrix::new([[2., 3., 4.], [5., 6., 7.]]));
        let z = x * y;
        assert_eq!(z.data(), Matrix::new([[12., 15., 18.], [26., 33., 40.]]));
        assert_eq!(
            z.creator().unwrap(),
            Function::MatMul(x.data_idx, y.data_idx)
        );
        assert_eq!(x.generation(), 0);
        assert_eq!(y.generation(), 0);
        assert_eq!(z.generation(), 1);
    }

    #[test]
    fn scalarmul() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[1., 2., 3.]]));
        let y: VariableHandle<'_, f64> = 2. * x;
        assert_eq!(y.data(), Matrix::new([[2., 4., 6.]]));
        assert_eq!(y.creator().unwrap(), Function::ScalarMul(x.data_idx, 1));
        let z = x * 3.;
        assert_eq!(z.data(), Matrix::new([[3., 6., 9.]]));
        assert_eq!(z.creator().unwrap(), Function::ScalarMul(x.data_idx, 3));
        assert_eq!(x.generation(), 0);
        assert_eq!(y.generation(), 1);
        assert_eq!(z.generation(), 1);
    }

    #[test]
    fn scalardiv() {
        let tape = VariableTape::new();
        let x = tape.variable(Matrix::new([[2., 4., 6.]]));
        let y = x / 2.;
        assert_eq!(y.data(), Matrix::new([[1., 2., 3.,]]));
        assert_eq!(y.creator().unwrap(), Function::ScalarDiv(x.data_idx, 1));
        assert_eq!(x.generation(), 0);
        assert_eq!(y.generation(), 1);
    }
}
