pub struct ROMFile<T> {
    pub rom: T,
}

pub trait ROMFs {
    fn new<'a>(rom_path: String) -> Self;
    fn read_file(rom_path: &str) -> Vec<u8>;
    fn read_rom_header(&self, header_size: usize) -> Vec<u8>;
    fn read_rom_content(&self) -> Vec<u8>;
    fn read_exact_at(&self, offset: usize, size: usize) -> Vec<u8>;
}

impl<T: ROMFs> ROMFile<T> {
    pub fn new(rom_path: String) -> Self {
        let rom = T::new(rom_path);

        ROMFile { rom }
    }
}
