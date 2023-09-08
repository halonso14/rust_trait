use crate::module_b::ErrorB;

#[derive(Debug)]
pub enum ErrorA {
    ErrorA1(String),
    ErrorA2(String),
    ErrorFromB(String),
    Unmanageable(String),
}

impl From<ErrorB> for ErrorA {
    fn from(error: ErrorB) -> ErrorA {
        match error {
            ErrorB::Manageable(error_message) => ErrorA::ErrorFromB(format!(
                "Error from do_a while using do_b. Error: {}",
                error_message
            )),
            ErrorB::Unmanageable(error_message) => ErrorA::Unmanageable(format!(
                "Error from do_a while using do_b. Unexpected Error: {}",
                error_message
            )),
        }
    }
}
