use std::fmt;

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    FiberState,
    Nil,
}

impl RuntimeError {
    #[cold]
    pub fn new<S>(kind: ErrorKind, message: S) -> Self
    where
        S: ToString,
    {
        Self {
            kind,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "runtime error: ")?;
        match self.kind {
            ErrorKind::FiberState => write!(f, "invalid fiber state: {}", self.message),
            ErrorKind::Nil => write!(f, "nil value"),
        }
    }
}
