#[derive(Debug)]
pub enum RegisterUserError {
    BadRequestError(Vec<String>),
    EmailIsTaken,
    InternalServerError,
}
