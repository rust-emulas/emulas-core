use crate::sys::{
    errors::{Extension, FileErrors},
    interfaces::ROMFs,
};

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

const DEFAULT_NES_ROM_HEADER: [u8; 4] = [78, 69, 83, 26]; // [N, E, S, 1A]

#[derive(Debug)]
pub struct ROMFileLinux {
    rom_path: String,
    content: Vec<u8>,
}

impl ROMFs for ROMFileLinux {
    fn new(rom_path: String) -> Result<Self, FileErrors> {
        Self::validate_file(&rom_path)?;

        let content = Self::read_file(&rom_path)?;

        Ok(ROMFileLinux { rom_path: rom_path, content: content})
    }

    fn validate_file(rom_path: &str) -> Result<(), FileErrors> {
        let rom_path = Path::new(rom_path);

        if !rom_path.is_file() {
            return Err(FileErrors::ErrorInvalidROMFile);
        }

        match rom_path.extension() {
            None => return Err(FileErrors::ErrorInvalidExtension),
            Some(extension) => {
                let extension = Extension::from_str(extension.to_str().unwrap_or(""));

                if extension == Extension::InvalidExtension {
                    return Err(FileErrors::ErrorInvalidExtension);
                }
            }
        }

        Ok(())
    }

    fn read_file(rom_path: &str) -> Result<Vec<u8>, FileErrors> {
        let mut buffer = Vec::new();

        match File::open(rom_path) {
            Ok(mut f) => {
                match f.read_to_end(&mut buffer) {
                    Ok(_) => {
                        if buffer.len() <= 16 {
                            Err(FileErrors::ErrorInvalidFileSize)
                        } else if buffer[0..4] != DEFAULT_NES_ROM_HEADER {
                            Err(FileErrors::ErrorInvalidROMFile)
                        } else {
                            Ok(buffer)
                        }
                    }
                    Err(_) => Err(FileErrors::ErrorReadingROMFile),
                }
            }
            Err(_) => Err(FileErrors::ErrorOpeningROMFile),
        }
    }

    fn read_exact_at(&self, offset: usize, size: usize) -> Result<&[u8], FileErrors> {
        self.content.get(offset..offset + size).ok_or(FileErrors::ErrorInvalidRange)
    }

    fn path(&self) -> &str {
        &self.rom_path
    }

    fn size(&self) -> usize {
        self.content.len()
    }
}
