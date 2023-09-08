use crate::common::TestCase;

use super::error::ExternalError;

// extenal module
pub fn do_external(case: &TestCase) -> Result<bool, ExternalError> {
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
        _ => (),
    }
    Ok(true)
}
