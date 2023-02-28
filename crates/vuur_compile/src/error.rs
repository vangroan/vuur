use std::fmt;

pub type Result<T> = std::result::Result<T, CompileError>;

#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    Compiler,
    Decode,
    Encode,
    Disassemble,
    Io(std::io::Error),
    Fmt(std::fmt::Error),
}

impl CompileError {
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

impl fmt::Display for CompileError {
    #[cold]
    #[inline(never)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "compile error: ")?;
        match self.kind {
            ErrorKind::Compiler => write!(f, "failed to compile: {}", self.message),
            ErrorKind::Decode => write!(f, "failed to decode bytes: {}", self.message),
            ErrorKind::Encode => write!(f, "failed to encode value: {}", self.message),
            ErrorKind::Disassemble => write!(f, "failed to disassemble bytecode: {}", self.message),
            ErrorKind::Io(ref err) => write!(f, "{}: {}", self.message, err),
            ErrorKind::Fmt(ref err) => write!(f, "{}: {}", self.message, err),
        }
    }
}

impl std::error::Error for CompileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }
}

impl From<std::io::Error> for CompileError {
    fn from(err: std::io::Error) -> Self {
        Self {
            message: "unexpected IO error".to_owned(),
            kind: ErrorKind::Io(err),
        }
    }
}

impl From<std::fmt::Error> for CompileError {
    fn from(err: std::fmt::Error) -> Self {
        Self {
            message: "unexpected formatting error".to_owned(),
            kind: ErrorKind::Fmt(err),
        }
    }
}

impl<T> Into<self::Result<T>> for CompileError {
    fn into(self) -> self::Result<T> {
        Err(self)
    }
}
