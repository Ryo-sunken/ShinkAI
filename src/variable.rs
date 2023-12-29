use matrix::matrix::{Matrix, Axis};

pub(crate) type Index = usize;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum Function<T>
{
    Add(Index, Index),
    Sub(Index, Index),
    MatMul(Index, Index),
    ScalarMul(Index, T),
    CWiseMul(Index, Index),
    CWiseDiv(Index, Index),
    Transpose(Index),
    Pow(Index, T),
    Exp(Index),
    Min(Index, Axis),
    Max(Index, Axis),
    Sum(Index, Axis),
}

#[derive(Clone, Debug)]
pub(crate) struct Variable<T>
{
    pub(crate) data: Matrix<T>,
    pub(crate) grad: Option<Index>,
    pub(crate) creator: Option<Function<T>>,
}

impl<T> Variable<T>
{
    pub(crate) fn new(data: Matrix<T>) -> Self
    {
        Self
        {
            data,
            grad: None,
            creator: None,
        }
    }

    pub(crate) fn from_function(data: Matrix<T>, creator: Function<T>) -> Self
    {
        Self
        {
            data,
            grad: None,
            creator: Some(creator),
        }
    }
}