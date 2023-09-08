use crate::common::{GenenricResponse, ToFailure, ToSuccess};
use serde::{Deserialize, Serialize};

pub struct ExternalResponse {
    pub error: bool,
}

impl From<ExternalResponse> for SuccessResponse {
    fn from(value: ExternalResponse) -> Self {
        SuccessResponse {
            error: 0,
            success: value.error,
        }
    }
}

impl From<ExternalResponse> for FailuerResponse {
    fn from(value: ExternalResponse) -> Self {
        FailuerResponse {
            error: 0,
            failure: value.error,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessResponse {
    error: u32,
    success: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FailuerResponse {
    error: u32,
    failure: bool,
}

impl ToSuccess<SuccessResponse, FailuerResponse> for SuccessResponse {
    fn to_success(self) -> GenenricResponse<SuccessResponse, FailuerResponse> {
        GenenricResponse::<SuccessResponse, FailuerResponse>::Success(self)
    }
}

impl ToSuccess<SuccessResponse, FailuerResponse> for FailuerResponse {
    fn to_success(self) -> GenenricResponse<SuccessResponse, FailuerResponse> {
        todo!();
    }
}

impl ToFailure<SuccessResponse, FailuerResponse> for SuccessResponse {
    fn to_failure(self) -> GenenricResponse<SuccessResponse, FailuerResponse> {
        todo!();
    }
}

impl ToFailure<SuccessResponse, FailuerResponse> for FailuerResponse {
    fn to_failure(self) -> GenenricResponse<SuccessResponse, FailuerResponse> {
        GenenricResponse::<SuccessResponse, FailuerResponse>::Failure(self)
    }
}

pub fn to_generic_response<S, F>(data: ExternalResponse) -> GenenricResponse<S, F>
where
    S: for<'de> serde::Deserialize<'de>
        + From<ExternalResponse>
        + ToSuccess<S, F>
        + ToFailure<S, F>,
    F: for<'de> serde::Deserialize<'de>
        + From<ExternalResponse>
        + ToSuccess<S, F>
        + ToFailure<S, F>,
{
    // Need logic to distinguish whether ExternalResponse is Success or Failure
    match data.error {
        true => Into::<S>::into(data).to_success(),
        false => Into::<F>::into(data).to_failure(),
    }
}
