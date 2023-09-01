use crate::module_b::ErrorB;

#[derive(Debug)]
pub enum ErrorA {
    ErrorA1(String),
    ErrorA2(String),
    ErrorFromB(String),
    ExternalError(String),
}

impl From<ErrorB> for ErrorA {
    fn from(error: ErrorB) -> ErrorA {
        match error {
            ErrorB::ErrorB1(error_message) => ErrorA::ErrorFromB(format!(
                "Error from do_a while using do_b. Error: {}",
                error_message
            )),
            ErrorB::ErrorB2(error_message) => ErrorA::ErrorFromB(format!(
                "Error from do_a while using do_b. Error: {}",
                error_message
            )),
            ErrorB::ExternalError(error_message) => ErrorA::ExternalError(format!(
                "Error from do_a while using do_external. Error: {}",
                error_message
            )),
        }
    }
}
