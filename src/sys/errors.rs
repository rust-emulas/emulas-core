use std::fmt;

#[derive(Debug, PartialEq)]
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

pub struct ErrorInvalidROMFile;
pub struct ErrorInvalidExtension;
pub struct ErrorInvalidRange;
pub struct ErrorOpeningROMFile;
pub struct ErrorReadingROMFile;

#[derive(Debug, PartialEq)]
pub enum FileErrors {
    ErrorInvalidROMFile,
    ErrorInvalidExtension,
    ErrorInvalidRange,
    ErrorInvalidFileSize,
    ErrorOpeningROMFile,
    ErrorReadingROMFile,
}

impl std::fmt::Display for FileErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileErrors::ErrorInvalidROMFile => write!(f, "File is not a valid ROM file."),
            FileErrors::ErrorInvalidExtension => write!(f, "File has an invalid extension."),
            FileErrors::ErrorInvalidRange => write!(f, "Invalid size for ROM content"),
            FileErrors::ErrorInvalidFileSize => write!(f, "File has a invalid size for ROM content"),
            FileErrors::ErrorOpeningROMFile => write!(f, "Error when try to open file."),
            FileErrors::ErrorReadingROMFile => write!(f, "Error when try to read file."),
        }
    }
}
