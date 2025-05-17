use std::fmt;

pub struct InvalidROMFile;
pub struct ErrorOnOpen;
pub struct ErrorOnRead;

// TODO improve error names
#[derive(Debug)]
pub enum FileErrors {
    InvalidROMFile,
    ErrorOnOpen,
    ErrorOnRead,
}

// TODO transforme this in generic
impl fmt::Display for InvalidROMFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File invalid as NES ROM!")
    }
}

impl fmt::Debug for InvalidROMFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ File: {} at line {} }}", file!(), line!())
    }
}

impl fmt::Display for ErrorOnOpen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error when tried to open ROM file")
    }
}

impl fmt::Debug for ErrorOnOpen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ File: {} at line {} }}", file!(), line!())
    }
}

impl fmt::Display for ErrorOnRead {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error when tried to read ROM file")
    }
}

impl fmt::Debug for ErrorOnRead {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ File: {} at line {} }}", file!(), line!())
    }
}
