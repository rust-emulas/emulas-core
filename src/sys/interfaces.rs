use super::errors::FileErrors;

pub struct ROMFile<T> {
    pub rom: T,
}

pub trait ROMFs {
    fn new(rom_path: String) -> Result<Self, FileErrors> where Self: Sized;
    fn validate_file(rom_path: &str) -> Result<(), FileErrors>;
    fn read_file(rom_path: &str) -> Result<Vec<u8>, FileErrors>;
    fn read_rom_header(&self, header_size: usize) -> Vec<u8>;
    fn read_rom_content(&self) -> Vec<u8>;
    fn read_exact_at(&self, offset: usize, size: usize) -> Vec<u8>;
}

impl<T: ROMFs> ROMFile<T> {
    pub fn new(rom_path: String) -> Result<Self, FileErrors> {
        let rom = T::new(rom_path)?;

        Ok(ROMFile { rom })
    }
}
