use crate::common::{GenenricResponse, TestCase};

use super::{
    error::ExternalError,
    response::{to_generic_response, ExternalResponse, FailuerResponse, SuccessResponse},
};

// extenal module
pub fn do_external(
    case: &TestCase,
) -> Result<GenenricResponse<SuccessResponse, FailuerResponse>, ExternalError> {
    match case {
        // error from external module
        TestCase::FailFromExternal1 => {
            return Err(ExternalError::Manageable(
                "Failed from exteranl 1".to_string(),
            ))
        }
        // error from external module
        TestCase::FailFromExternal2 => {
            return Err(ExternalError::Unmanageable(
                "Failed from exteranl 2".to_string(),
            ))
        }
        TestCase::Failure => {
            let external_response_false = ExternalResponse { error: false };
            println!("   ⇓  CASE: FailuerResponse");
            return Ok(to_generic_response::<SuccessResponse, FailuerResponse>(
                external_response_false,
            ));
        }
        _ => {
            let external_response_true = ExternalResponse { error: true };
            println!("   ⇓  CASE: SuccessResponse");
            return Ok(to_generic_response::<SuccessResponse, FailuerResponse>(
                external_response_true,
            ));
        }
    }
}
