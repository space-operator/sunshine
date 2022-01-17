// (data, Result<ok, err>)
// Result<(data, ok), (data, err)>

pub trait SplitResult {
    type Output;
    fn split(self) -> Self::Output;
}

pub trait JoinResult {
    type Output;
    fn join(self) -> Self::Output;
}

impl<T, Ok, Err> SplitResult for Result<(T, Ok), (T, Err)> {
    type Output = (T, Result<Ok, Err>);
    fn split(self) -> Self::Output {
        match self {
            Ok((data, ok)) => (data, Ok(ok)),
            Err((data, err)) => (data, Err(err)),
        }
    }
}

impl<T, Ok, Err> JoinResult for (T, Result<Ok, Err>) {
    type Output = Result<(T, Ok), (T, Err)>;
    fn join(self) -> Self::Output {
        match self {
            (data, Ok(ok)) => Ok((data, ok)),
            (data, Err(err)) => Err((data, err)),
        }
    }
}
