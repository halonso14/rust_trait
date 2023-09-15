use crate::external_module::ExternalError;

#[derive(Debug)]
pub enum ErrorB {
    Manageable(String),
    Unmanageable(String),
}

impl From<ExternalError> for ErrorB {
    fn from(error: ExternalError) -> ErrorB {
        // Map external error to proper error
        match error {
            ExternalError::Manageable(error_message) => ErrorB::Manageable(format!(
                "Error from do_b while using do_external. Error: {}",
                error_message
            )),
            ExternalError::Unmanageable(error_message) => ErrorB::Unmanageable(format!(
                "Error from do_b while using do_external. Error: {}",
                error_message
            )),
        }
    }
}
