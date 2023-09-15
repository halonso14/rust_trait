#[derive(Debug)]
pub enum TestCase {
    Success,
    Failure,
    FailFromA1,
    FailFromA2,
    FailFromB1,
    FailFromB2,
    FailFromExternal1,
    FailFromExternal2,
}

pub enum GenenricResponse<S, F> {
    Success(S),
    Failure(F),
}

pub trait ToSuccess<S, F> {
    fn to_success(self) -> GenenricResponse<S, F>;
}

pub trait ToFailure<S, F> {
    fn to_failure(self) -> GenenricResponse<S, F>;
}
