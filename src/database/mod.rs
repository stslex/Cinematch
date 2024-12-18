use std::fmt::Display;

pub mod user;

pub enum ErrorResponseDb {
    INTERNAL_SERVER_ERROR,
    NOT_FOUND,
    CONFLICT,
}

impl Display for ErrorResponseDb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorResponseDb::INTERNAL_SERVER_ERROR => write!(f, "Internal server error"),
            ErrorResponseDb::NOT_FOUND => write!(f, "Not found"),
            ErrorResponseDb::CONFLICT => write!(f, "Conflict"),
        }
    }
}
