use std::fmt;

#[derive(Debug)]
pub enum Extension {
    NES,
    InvalidExtension,
}

impl Extension {
    pub fn from_str(ext: &str) -> Self {
        match ext.to_uppercase().as_str() {
            "NES" => Extension::NES,
            _ => Extension::InvalidExtension,
        }
    }
}

impl PartialEq for Extension {
   fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Extension::NES, Extension::NES) => true,
            _ => false,
        }
    }
}

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
