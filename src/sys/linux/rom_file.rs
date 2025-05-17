use crate::sys::{
    interfaces::{
        ROMFs,
        Extension,
    },
    errors::FileErrors,
    errors::ErrorOnRead,
};

use std::fs::File;
use std::io::Read;
use std::io;
use std::path::Path;

pub struct ROMFileLinux {
    pub rom_path: String,
    pub content: Vec<u8>,
    pub size: usize,
}

impl ROMFs for ROMFileLinux {
    fn new(rom_path: String) -> Result<Self, FileErrors> {
        let mut rom = ROMFileLinux {
            rom_path: rom_path.clone(),
            content: Vec::new(),
            size: 0,
        };

        match Self::validate_file(&rom_path) {
            Ok(_) => {
                rom.content = Self::read_file(&rom_path)?;
                rom.size = rom.content.len();

                Ok(rom)
            }
            Err(err) => Err(FileErrors::InvalidROMFile),
        }

    }

    fn validate_file(rom_path: &str) -> Result<(), FileErrors> {
        let rom_path = Path::new(rom_path);
        let mut valid = true;

        // validate if the path is actualy a file 
        if !rom_path.is_file() {
            valid = false
        }

        // validate the file extension
        match rom_path.extension() {
            None => valid = false,
            Some(extension) => {
                if Extension::from_str(extension.to_str().unwrap_or("")) != Extension::NES {
                    valid = false;
                }
            }
        }

        if valid {
            Ok(())
        } else {
            Err(FileErrors::InvalidROMFile)
        }
    }

    fn read_file(rom_path: &str) -> Result<Vec<u8>, FileErrors> {
        let mut buffer = Vec::new();

        match File::open(rom_path) {
            Ok(mut f) => {
                // erro ao ler
                match f.read_to_end(&mut buffer) {
                    Ok(_) => Ok(buffer),
                    Err(_) => Err(FileErrors::ErrorOnRead),
                }
            }
            Err(_) => Err(FileErrors::ErrorOnOpen),
        }

    }

    fn read_rom_header(&self, header_size: usize) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        for b in &self.content[0..header_size] {
            buf.push(*b);
        }

        buf.clone()
    }

    fn read_rom_content(&self) -> Vec<u8> {
        self.content[0..self.size].to_vec()
    }

    fn read_exact_at(&self, offset: usize, size: usize) -> Vec<u8> {
        self.content[offset..size].to_vec()
    }
}

