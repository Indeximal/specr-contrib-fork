use std::ops::{Try, FromResidual, ControlFlow, Residual, Yeet};
use std::convert::Infallible;

/// Conceptually, this is a `Nondet<Result<T, E>>`.
///
/// This newtype is necessary so that applying `?` on a `NdResult<T, E>` yields `T` and not `Result<T, E>`.
#[must_use]
pub struct NdResult<T, E>(pub(crate) Result<T, E>);

impl<T, E> NdResult<T, E> {
    #[doc(hidden)]
    pub fn from_internal(r: Result<T, E>) -> Self { Self(r) }

    #[doc(hidden)]
    pub fn get_internal(self) -> Result<T, E> {
        self.0
    }

    /// Monadic bind: chains `f` after the current result, if this is not already an error.
    pub fn and_then<U>(self, f: impl FnOnce(T) -> NdResult<U, E>) -> NdResult<U, E> {
        NdResult(self.0.and_then(|x| f(x).0))
    }
}

impl<T, E> Try for NdResult<T, E> {
    type Output = T;
    type Residual = NdResult<Infallible, E>;

    fn from_output(output: Self::Output) -> Self {
        NdResult(Ok(output))
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(x) => ControlFlow::Continue(x),
            Err(e) => ControlFlow::Break(NdResult(Err(e))),
        }
    }
}

// in order to use `?`` on NdResults in an NdResult-returning fn.
impl<T, E> FromResidual<NdResult<Infallible, E>> for NdResult<T, E> {
    fn from_residual(residual: NdResult<Infallible, E>) -> Self {
        match residual.0 {
            Ok(x) => match x {},
            Err(e) => NdResult(Err(e))
        }
    }
}

// in order to use `?` on Results in a NdResult-returning fn.
impl<T, E> FromResidual<Result<Infallible, E>> for NdResult<T, E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Ok(x) => match x {},
            Err(e) => NdResult(Err(e))
        }
    }
}

// in order to use `?` on Nondet in a NdResult-returning fn.
impl<T, E> FromResidual<Infallible> for NdResult<T, E> {
    fn from_residual(residual: Infallible) -> Self {
        match residual {}
    }
}

// required by try_collect
impl<T, E> Residual<T> for NdResult<Infallible, E> {
    type TryType = NdResult<T, E>;
}

// in order to yeet in a NdResult-returning fn.
impl<T, E> FromResidual<Yeet<E>> for NdResult<T, E> {
    fn from_residual(residual: Yeet<E>) -> Self {
        NdResult(Err(residual.0))
    }
}
