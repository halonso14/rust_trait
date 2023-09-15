#[derive(Debug)]
pub enum ExternalError {
    Manageable(String),
    Unmanageable(String),
}
