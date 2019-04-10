use futures::{Future, Map, MapErr};

pub trait ResultMapInto<U> {
    type Output;
    fn map_into(self) -> Self::Output;
}

pub trait ResultMapErrInto<U> {
    type Output;
    fn map_err_into(self) -> Self::Output;
}

impl<T, E, U: From<T>> ResultMapInto<U> for Result<T, E> {
    type Output = Result<U, E>;
    fn map_into(self) -> Self::Output {
        self.map(Into::into)
    }
}

impl<T, E, U: From<E>> ResultMapErrInto<U> for Result<T, E> {
    type Output = Result<T, U>;
    fn map_err_into(self) -> Self::Output {
        self.map_err(Into::into)
    }
}

pub trait FutureMapInto<U> {
    type Output;
    fn map_into(self) -> Self::Output;
}

pub trait FutureMapErrInto<U> {
    type Output;
    fn map_err_into(self) -> Self::Output;
}
impl<F, U> FutureMapInto<U> for F
    where F: Future,
          U: From<F::Item>
{
    type Output = Map<F, fn(F::Item) -> U>;
    fn map_into(self) -> Self::Output {
        self.map(Into::into)
    }
}

impl<F, U> FutureMapErrInto<U> for F
    where F: Future,
          U: From<F::Error>
{
    type Output = MapErr<F, fn(F::Error) -> U>;
    fn map_err_into(self) -> Self::Output {
        self.map_err(Into::into)
    }
}
