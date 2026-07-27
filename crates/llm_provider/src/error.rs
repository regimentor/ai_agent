use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Network(String),
    InvalidResponse(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred")
    }
}
