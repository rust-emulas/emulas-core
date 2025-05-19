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
            (Extension::InvalidExtension, Extension::InvalidExtension) => true,
            _ => false,
        }
    }
}

pub struct InvalidROMFile;
pub struct ErrorOpeningROMFile;
pub struct ErrorReadingROMFile;

// TODO improve error names
#[derive(Debug)]
pub enum FileErrors {
    InvalidROMFile,
    ErrorOpeningROMFile,
    ErrorReadingROMFile,
}

trait FileErrorMessage {}

impl fmt::Display for dyn FileErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File invalid as NES ROM!")
    }
}

impl fmt::Debug for dyn FileErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ File: {} at line {} }}", file!(), line!())
    }
}

impl FileErrorMessage for InvalidROMFile {}
impl FileErrorMessage for ErrorOpeningROMFile {}
impl FileErrorMessage for ErrorReadingROMFile {}
