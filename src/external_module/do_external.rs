use crate::common::TestCase;

use super::error::ExternalError;

// extenal module
pub fn do_external(case: &TestCase) -> Result<bool, ExternalError> {
    match case {
        // error from external module
        TestCase::FailFromBottom1 => {
            return Err(ExternalError::ExternalError1(
                "Failed from bottom 1".to_string(),
            ))
        }
        // error from external module
        TestCase::FailFromBottom2 => {
            return Err(ExternalError::ExternalError2(
                "Failed from bottom 2".to_string(),
            ))
        }
        _ => (),
    }
    Ok(true)
}
