use crate::external_module::ExternalError;

#[derive(Debug)]
pub enum ErrorB {
    ErrorB1(String),
    ErrorB2(String),
    ExternalError(String),
}

impl From<ExternalError> for ErrorB {
    fn from(error: ExternalError) -> ErrorB {
        // Map external error to proper error
        match error {
            ExternalError::ExternalError1(error_message) => ErrorB::ExternalError(format!(
                "Error from do_b while using do_external. Error: {}",
                error_message
            )),
            ExternalError::ExternalError2(error_message) => ErrorB::ExternalError(format!(
                "Error from do_b while using do_external. Error: {}",
                error_message
            )),
        }
    }
}
