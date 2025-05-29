use super::errors::FileErrors;

#[derive(Debug)]
pub struct ROMFile<T> {
    rom: T,
}

pub trait ROMFs {
    fn new(rom_path: String) -> Result<Self, FileErrors>
    where
        Self: Sized;
    fn validate_file(rom_path: &str) -> Result<(), FileErrors>;
    fn read_file(rom_path: &str) -> Result<Vec<u8>, FileErrors>;
    fn read_exact_at(&self, offset: usize, size: usize) -> Result<&[u8], FileErrors>;
    fn read_rom_header(&self) -> Result<[u8; 16], FileErrors>;
    fn path(&self) -> &str;
    fn size(&self) -> usize;
}

#[derive(Debug)]
pub struct INes {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub trainer: usize,
    pub prg_size: usize,
    pub chr_size: usize,
    pub mapper: u8,
    pub mirroring: MirroringType,
}

impl Default for INes {
    fn default() -> Self {
        INes {
            prg_rom: Vec::new(),
            chr_rom: Vec::new(),
            trainer: 0,
            prg_size: 0,
            chr_size: 0,
            mapper: 0,
            mirroring: MirroringType::Horizontal,
        }
    }
}

#[derive(Debug, Clone)]
#[repr(u8)]
pub enum MirroringType {
    Horizontal,
    Vertical,
    FourScreen,
}

impl<T: ROMFs> ROMFile<T> {
    pub fn new(rom_path: String) -> Result<Self, FileErrors> {
        let rom = T::new(rom_path)?;

        Ok(ROMFile { rom })
    }

    pub fn path(&self) -> &str {
        self.rom.path()
    }

    pub fn size(&self) -> usize {
        self.rom.size()
    }

    pub fn read_rom_content(&self) -> Result<&[u8], FileErrors> {
        Ok(&self.rom.read_exact_at(0, self.size())?)
    }

    pub fn read_exact_at(&self, offset: usize, size: usize) -> Result<&[u8], FileErrors> {
        Ok(&self.rom.read_exact_at(offset, size)?)
    }

    pub fn read_rom_header(&self) -> Result<[u8; 16], FileErrors> {
        Ok(self.rom.read_rom_header()?)
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs::File, io::Write};

    use crate::sys::rom_file::ROM;

    use super::*;

    fn create_temp_rom_file(content: &[u8], ext: &str) -> String {
        let dir = env::temp_dir();
        let file_path = dir.join(format!("test_rom_file{}", ext));
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content).unwrap();
        file_path.to_str().unwrap().to_string()
    }

    #[test]
    fn test_validate_file_valid() {
        let path = create_temp_rom_file(&[0u8; 16], ".nes");
        assert!(ROM::validate_file(&path).is_ok());
    }

    #[test]
    fn test_validate_file_invalid_extension() {
        let path = create_temp_rom_file(&[0u8; 16], ".txt");
        let err = ROM::validate_file(&path).unwrap_err();
        assert_eq!(err, FileErrors::ErrorInvalidExtension);
    }

    #[test]
    fn test_validate_file_nonexistent() {
        let err = ROM::validate_file("nonexistent.nes").unwrap_err();
        assert_eq!(err, FileErrors::ErrorInvalidROMFile);
    }

    #[test]
    fn test_read_file_success() {
        let data = [1, 2, 3, 4, 5];
        let path = create_temp_rom_file(&data, ".nes");
        let read = ROM::read_file(&path).unwrap();
        assert_eq!(&read[..5], data);
    }

    #[test]
    fn test_read_file_failure() {
        let err = ROM::read_file("does_not_exist.nes").unwrap_err();
        assert_eq!(err, FileErrors::ErrorOpeningROMFile);
    }

    #[test]
    fn test_read_exact_at_success() {
        let rom = ROM {
            format: INes::default(),
            rom_path: "dummy".to_string(),
            content: vec![10, 20, 30, 40, 50],
        };
        let slice = rom.read_exact_at(1, 3).unwrap();
        assert_eq!(slice, &[20, 30, 40]);
    }

    #[test]
    fn test_read_exact_at_out_of_bounds() {
        let rom = ROM {
            format: INes::default(),
            rom_path: "dummy".to_string(),
            content: vec![1, 2, 3],
        };
        let err = rom.read_exact_at(2, 5).unwrap_err();
        assert_eq!(err, FileErrors::ErrorInvalidRange);
    }

    #[test]
    fn test_read_rom_header_success() {
        let mut content = vec![0u8; 32];
        content[0..16].copy_from_slice(&[1u8; 16]);
        let rom = ROM {
            format: INes::default(),
            rom_path: "dummy".to_string(),
            content,
        };
        let header = rom.read_rom_header().unwrap();
        assert_eq!(header, [1u8; 16]);
    }

    #[test]
    fn test_read_rom_header_too_small() {
        let rom = ROM {
            format: INes::default(),
            rom_path: "dummy".to_string(),
            content: vec![1u8; 10],
        };
        let err = rom.read_rom_header().unwrap_err();
        assert_eq!(err, FileErrors::ErrorInvalidRange);
    }

    #[test]
    fn test_path() {
        let rom = ROM {
            format: INes::default(),
            rom_path: "abc/def.nes".to_string(),
            content: vec![],
        };
        assert_eq!(rom.path(), "abc/def.nes");
    }

    #[test]
    fn test_size() {
        let rom = ROM {
            format: INes::default(),
            rom_path: "abc".to_string(),
            content: vec![1, 2, 3, 4],
        };
        assert_eq!(rom.size(), 4);
    }

    #[test]
    fn test_new_success() {
        let mut content = vec![0u8; 16 + 16 * 1024];
        content[0..4].copy_from_slice(b"NES\x1A");
        content[4] = 1; // 1 PRG block
        content[5] = 0; // 0 CHR block
        let path = create_temp_rom_file(&content, ".nes");
        let rom = ROM::new(path.clone());
        assert!(rom.is_ok());
    }

    #[test]
    fn test_new_invalid_file() {
        let rom = ROM::new("does_not_exist.nes".to_string());
        assert!(rom.is_err());
    }
}
