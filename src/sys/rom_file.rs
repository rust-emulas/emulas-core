use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::sys::{
    errors::{Extension, FileErrors},
    interfaces::ROMFs,
};

use super::interfaces::{INes, MirroringType};

const DEFAULT_NES_ROM_HEADER: [u8; 4] = [b'N', b'E', b'S', 0x1A]; // [N, E, S, 1A]

#[derive(Debug)]
pub struct ROM {
    pub format: INes,
    pub(crate) rom_path: String,
    pub(crate) content: Vec<u8>,
}

impl ROMFs for ROM {
    fn new(rom_path: String) -> Result<Self, FileErrors> {
        Self::validate_file(&rom_path)?;

        let content = Self::read_file(&rom_path)?;
        let format = Self::parse_ines(&content)?;

        Ok(ROM {
            rom_path,
            content,
            format,
        })
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
                f.read_to_end(&mut buffer)
                    .map_err(|_| FileErrors::ErrorOpeningROMFile)?;
                Ok(buffer)
            }
            Err(_) => Err(FileErrors::ErrorOpeningROMFile),
        }
    }

    fn read_exact_at(&self, offset: usize, size: usize) -> Result<&[u8], FileErrors> {
        let end = offset
            .checked_add(size)
            .ok_or(FileErrors::ErrorInvalidRange)?;

        if end > self.content.len() || offset > self.content.len() {
            return Err(FileErrors::ErrorInvalidRange);
        }

        Ok(&self.content[offset..end])
    }
    fn read_rom_header(&self) -> Result<[u8; 16], FileErrors> {
        let header = self.read_exact_at(0, 16)?;

        if header.len() < 16 {
            return Err(FileErrors::ErrorInvalidFileSize);
        }

        let mut rom_header = [0; 16];
        rom_header.copy_from_slice(header);

        Ok(rom_header)
    }
    fn path(&self) -> &str {
        &self.rom_path
    }

    fn size(&self) -> usize {
        self.content.len()
    }
}

impl ROM {
    fn parse_ines(content: &[u8]) -> Result<INes, FileErrors> {
        let header = &content[0..16];
        if header.len() < 16 || &header[0..4] != DEFAULT_NES_ROM_HEADER {
            return Err(FileErrors::ErrorInvalidFileSize);
        }

        let prg_blocks = header[4] as usize;
        let chr_blocks = header[5] as usize;

        let prg_size = prg_blocks * 16 * 1024;
        let chr_size = chr_blocks * 8 * 1024;

        let has_trainer = header[6] & 0b00000100 != 0;
        let trainer_size = if has_trainer { 512 } else { 0 };

        let prg_start = 16 + trainer_size;
        let prg_end = prg_start + prg_size;
        let chr_start = prg_end;
        let chr_end = chr_start + chr_size;

        if chr_end > content.len() || prg_end > content.len() {
            return Err(FileErrors::ErrorInvalidFileSize);
        }

        let trainer = if has_trainer { 512 } else { 0 };
        let prg_rom = content[prg_start..prg_end].to_vec();
        let chr_rom = content[chr_start..chr_end].to_vec();

        let mirroring = match header[6] & 0b00001001 {
            0x08 => MirroringType::FourScreen,
            0x01 => MirroringType::Vertical,
            _ => MirroringType::Horizontal,
        };

        let mapper = ((header[7] & 0xF0) | (header[6] >> 4)) as u8;

        Ok(INes {
            prg_rom,
            chr_rom,
            trainer,
            prg_size,
            chr_size,
            mapper,
            mirroring,
        })
    }
}
