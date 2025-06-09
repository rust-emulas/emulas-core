#[derive(Debug, PartialEq)]
pub enum Extension {
    Nes,
    InvalidExtension,
}

impl Extension {
    pub fn from_str(ext: &str) -> Self {
        match ext.to_uppercase().as_str() {
            "NES" => Extension::Nes,
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
pub enum Error {
    ErrorInvalidROMFile,
    ErrorInvalidExtension,
    ErrorInvalidRange,
    ErrorInvalidFileSize,
    ErrorOpeningROMFile,
    ErrorReadingROMFile,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ErrorInvalidROMFile => write!(f, "File is not a valid ROM file."),
            Error::ErrorInvalidExtension => write!(f, "File has an invalid extension."),
            Error::ErrorInvalidRange => write!(f, "Invalid size for ROM content"),
            Error::ErrorInvalidFileSize => {
                write!(f, "File has a invalid size for ROM content")
            }
            Error::ErrorOpeningROMFile => write!(f, "Error when try to open file."),
            Error::ErrorReadingROMFile => write!(f, "Error when try to read file."),
        }
    }
}
