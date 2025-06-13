use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::{
    memory::{Bus, BusInterface},
    sys::errors::Error,
};
use log::info;

#[derive(Debug)]
pub struct ROMFile<T> {
    rom: T,
}

pub trait ROMFs<'a>: Sized {
    fn new<P: AsRef<Path>>(rom_path: &'a P) -> Result<Self, Error>
    where
        Self: Sized;
    fn write_rom_memory<B>(&self, bus: &mut B) -> Result<(), Error>
    where
        B: BusInterface,
        Self: Sized;
    fn validate_file<P: AsRef<Path>>(rom_path: P) -> Result<(), Error>;
    fn read_file<P: AsRef<Path>>(rom_path: P) -> Result<Vec<u8>, Error>;
    fn read_exact_at(&self, offset: usize, size: usize) -> Result<&[u8], Error>;
    fn get_header(&self) -> Result<HeaderBytes, Error>;
    fn path(&self) -> impl AsRef<Path>;
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MirroringType {
    Horizontal,
    Vertical,
    FourScreen,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeaderBytes(pub [u8; 16]);

impl std::fmt::Display for HeaderBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Deref for HeaderBytes {
    type Target = [u8; 16];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HeaderBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: for<'a> ROMFs<'a>> ROMFile<T> {
    pub fn new<'a, P: AsRef<Path>>(rom_path: &'a P) -> Result<Self, Error> {
        let rom = T::new(rom_path)?;

        Ok(ROMFile { rom })
    }

    pub fn path(&self) -> impl AsRef<Path> {
        self.rom.path()
    }

    pub fn size(&self) -> usize {
        info!("Getting ROM file size: {}", self.rom.size());
        self.rom.size()
    }

    pub fn read_rom_content(&self) -> Result<&[u8], Error> {
        info!("Reading ROM content of size: {}", self.size());
        Ok(&self.rom.read_exact_at(0, self.size())?)
    }

    pub fn read_exact_at(&self, offset: usize, size: usize) -> Result<&[u8], Error> {
        Ok(&self.rom.read_exact_at(offset, size)?)
    }

    pub fn get_header(&self) -> Result<HeaderBytes, Error> {
        let header = self.rom.get_header()?;
        info!("Reading ROM header {}", header);

        Ok(header)
    }

    pub fn write_rom_memory(&self, bus: &mut Bus) -> Result<(), Error> {
        info!("Writing ROM to path: {:?}", self.path().as_ref());
        Ok(self.rom.write_rom_memory(bus)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        memory::{MIRRORED_PRG_SIZE, PPU_SIZE, PRG_SIZE, RAM_SIZE},
        sys::rom_file::{DEFAULT_NES_ROM_HEADER, ROM},
    };
    use std::{
        fs::File,
        io::{Seek, SeekFrom, Write},
    };
    use tempfile::{NamedTempFile, tempdir};

    fn create_temp_rom_file(content: &[u8], ext: &str) -> NamedTempFile {
        let mut file = tempfile::Builder::new()
            .suffix(ext)
            .tempfile()
            .expect("failed to create temp file");

        file.write_all(content)
            .expect("failed to write to temp file");
        file.seek(SeekFrom::Start(0)).unwrap(); // importante para leitura posterior
        file
    }

    #[test]
    fn test_validate_file_invalid_path() {
        let result = ROM::validate_file("nonexistent.nes");
        println!("Result: {:?}", result);
        assert!(matches!(result, Err(Error::ErrorInvalidROMFile)));
    }

    #[test]
    fn test_validate_file_invalid_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();
        let result = ROM::validate_file(&file_path);
        assert!(matches!(result, Err(Error::ErrorInvalidExtension)));
    }

    #[test]
    fn test_read_file_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.nes");
        let data = vec![1, 2, 3, 4];
        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(&data).unwrap();
        }
        let result = ROM::read_file(&file_path).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_read_file_failure() {
        let result = ROM::read_file("does_not_exist.nes");
        assert!(matches!(result, Err(Error::ErrorOpeningROMFile)));
    }

    #[test]
    fn test_read_exact_at_valid() {
        let rom = ROM {
            format: INes::default(),
            rom_path: Path::new("dummy.nes"),
            content: vec![0, 1, 2, 3, 4, 5],
        };
        let slice = rom.read_exact_at(2, 3).unwrap();
        assert_eq!(slice, &[2, 3, 4]);
    }

    #[test]
    fn test_read_exact_at_invalid_range() {
        let rom = ROM {
            format: INes::default(),
            rom_path: Path::new("dummy.nes"),
            content: vec![0, 1, 2],
        };
        let result = rom.read_exact_at(2, 5);
        assert!(matches!(result, Err(Error::ErrorInvalidRange)));
    }

    #[test]
    fn test_get_header_success() {
        let mut content = vec![0u8; 32];
        content[0..4].copy_from_slice(DEFAULT_NES_ROM_HEADER);
        let rom = ROM {
            format: INes::default(),
            rom_path: Path::new("dummy.nes"),
            content,
        };
        let header = rom.get_header().unwrap();
        assert_eq!(&header.0[0..4], DEFAULT_NES_ROM_HEADER);
    }

    #[test]
    fn test_get_header_too_small() {
        let rom = ROM {
            format: INes::default(),
            rom_path: Path::new("dummy.nes"),
            content: vec![0u8; 10],
        };
        let result = rom.get_header();
        assert!(matches!(result, Err(Error::ErrorInvalidRange)));
    }

    #[test]
    fn test_parse_ines_invalid_header() {
        let content = vec![0u8; 16];
        let result = ROM::parse_ines(&content);
        assert!(matches!(result, Err(Error::ErrorInvalidROMFile)));
    }

    #[test]
    fn test_parse_ines_too_small() {
        let content = vec![0u8; 10];
        let result = ROM::parse_ines(&content);
        assert!(matches!(result, Err(Error::ErrorInvalidFileSize)));
    }

    #[test]
    fn test_parse_ines_valid() {
        let mut header = [0u8; 16];
        header[0..4].copy_from_slice(DEFAULT_NES_ROM_HEADER);
        header[4] = 1; // 1 PRG block (16KB)
        header[5] = 1; // 1 CHR block (8KB)
        let prg = vec![0xAA; 16 * 1024];
        let chr = vec![0xBB; 8 * 1024];
        let mut content = Vec::new();
        content.extend_from_slice(&header);
        content.extend_from_slice(&prg);
        content.extend_from_slice(&chr);

        let ines = ROM::parse_ines(&content).unwrap();
        assert_eq!(ines.prg_rom, prg);
        assert_eq!(ines.chr_rom, chr);
        assert_eq!(ines.prg_size, 16 * 1024);
        assert_eq!(ines.chr_size, 8 * 1024);
    }

    #[test]
    fn test_new_success() {
        let mut header = [0u8; 16];
        header[0..4].copy_from_slice(DEFAULT_NES_ROM_HEADER);
        header[4] = 1;
        header[5] = 1;
        let prg = vec![0xAA; 16 * 1024];
        let chr = vec![0xBB; 8 * 1024];

        // Compose the ROM file content: header + prg + chr
        let mut content = Vec::new();
        content.extend_from_slice(&header);
        content.extend_from_slice(&prg);
        content.extend_from_slice(&chr);

        let file = create_temp_rom_file(&content, ".nes");
        let file_path = file.path();

        let rom = ROM::new(&file_path).unwrap();
        assert_eq!(rom.content.len(), 16 + 16 * 1024 + 8 * 1024);
        assert_eq!(rom.format.prg_rom, prg);
        assert_eq!(rom.format.chr_rom, chr);
    }

    #[test]
    fn test_new_invalid_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("bad.nes");
        File::create(&file_path).unwrap();
        let result = ROM::new(&file_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_path_and_size() {
        let content = vec![1, 2, 3, 4, 5];
        let rom = ROM {
            format: INes::default(),
            rom_path: Path::new("foo.nes"),
            content: content.clone(),
        };
        assert_eq!(rom.size(), content.len());
        assert_eq!(rom.path().as_ref(), Path::new("foo.nes"));
    }

    fn dummy_header(prg_blocks: u8, chr_blocks: u8, flags6: u8, flags7: u8) -> [u8; 16] {
        let mut header = [0u8; 16];
        header[0..4].copy_from_slice(DEFAULT_NES_ROM_HEADER);
        header[4] = prg_blocks;
        header[5] = chr_blocks;
        header[6] = flags6;
        header[7] = flags7;
        header
    }

    #[test]
    fn test_parse_ines_valid_no_trainer() {
        let header = dummy_header(2, 1, 0, 0);
        let prg_size = 2 * 16 * 1024;
        let chr_size = 1 * 8 * 1024;
        let mut content = Vec::from(header);
        content.extend(vec![0xAA; prg_size]);
        content.extend(vec![0xBB; chr_size]);

        let ines = ROM::parse_ines(&content).unwrap();
        assert_eq!(ines.prg_rom.len(), prg_size);
        assert_eq!(ines.chr_rom.len(), chr_size);
        assert_eq!(ines.trainer, 0);
        assert_eq!(ines.mirroring, MirroringType::Horizontal);
    }

    #[test]
    fn test_parse_ines_valid_with_trainer() {
        let header = dummy_header(1, 1, 0b00000100, 0);
        let prg_size = 1 * 16 * 1024;
        let chr_size = 1 * 8 * 1024;
        let mut content = Vec::from(header);
        content.extend(vec![0xFF; 512]); // trainer
        content.extend(vec![0xAA; prg_size]);
        content.extend(vec![0xBB; chr_size]);

        let ines = ROM::parse_ines(&content).unwrap();
        assert_eq!(ines.trainer, 512);
        assert_eq!(ines.prg_rom.len(), prg_size);
        assert_eq!(ines.chr_rom.len(), chr_size);
    }

    #[test]
    fn test_parse_ines_invalid_file_size() {
        let header = dummy_header(2, 1, 0, 0);
        let mut content = Vec::from(header);
        // Not enough data for PRG/CHR
        content.extend(vec![0xAA; 100]);
        let result = ROM::parse_ines(&content);
        assert!(matches!(result, Err(Error::ErrorInvalidFileSize)));
    }

    #[test]
    fn test_parse_ines_mirroring_types() {
        // FourScreen
        let header = dummy_header(1, 1, 0x08, 0);
        let mut content = Vec::from(header);
        content.extend(vec![0; 16 * 1024 + 8 * 1024]);
        let ines = ROM::parse_ines(&content).unwrap();
        assert_eq!(ines.mirroring, MirroringType::FourScreen);

        // Vertical
        let header = dummy_header(1, 1, 0x01, 0);
        let mut content = Vec::from(header);
        content.extend(vec![0; 16 * 1024 + 8 * 1024]);
        let ines = ROM::parse_ines(&content).unwrap();
        assert_eq!(ines.mirroring, MirroringType::Vertical);

        // Horizontal (default)
        let header = dummy_header(1, 1, 0x00, 0);
        let mut content = Vec::from(header);
        content.extend(vec![0; 16 * 1024 + 8 * 1024]);
        let ines = ROM::parse_ines(&content).unwrap();
        assert_eq!(ines.mirroring, MirroringType::Horizontal);
    }

    #[test]
    fn test_parse_ines_mapper_number() {
        let header = dummy_header(1, 1, 0xF0, 0xF0);
        let mut content = Vec::from(header);
        content.extend(vec![0; 16 * 1024 + 8 * 1024]);
        let ines = ROM::parse_ines(&content).unwrap();
        // Mapper = (header[7] & 0xF0) | (header[6] >> 4)
        let expected_mapper = (0xF0 & 0xF0) | (0xF0 >> 4);
        assert_eq!(ines.mapper, expected_mapper as u8);
    }

    struct DummyBus {
        pub loaded: bool,
        pub last_data: Vec<u8>,
    }

    impl BusInterface for DummyBus {
        fn new(_rom: &[u8]) -> Self {
            Self {
                loaded: false,
                last_data: vec![],
            }
        }
        fn load_prg_rom(&mut self, data: &[u8]) -> Result<usize, Error> {
            self.loaded = true;
            self.last_data = data.to_vec();

            Ok(data.len())
        }

        fn resolve_prg_rom_index(&self, _addr: u16) -> usize {
            todo!()
        }

        fn write(&mut self, _addr: u16, _value: u8) {
            todo!()
        }

        fn read(&self, _addr: u16) -> u8 {
            todo!()
        }
    }

    impl Default for DummyBus {
        fn default() -> Self {
            Self::new(&[0])
        }
    }

    #[test]
    fn test_write_rom_loads_prg_rom_and_prints_info() {
        let mut content = vec![0u8; 0xFFFFF]; // 16KB PRG ROM
        content[0..4].copy_from_slice(DEFAULT_NES_ROM_HEADER);

        let rom_data = create_temp_rom_file(&content, ".nes");
        let tempfile = rom_data.path();
        let rom = ROM::new(&tempfile).unwrap();
        let mut bus = DummyBus::new(&content);

        // Actually test
        let result = rom.write_rom_memory(&mut bus);
        assert!(result.is_ok());
    }

    #[test]
    fn test_bus_new_initializes_memory() {
        let prg_data = vec![1, 2, 3, 4];
        let bus = Bus::new(&prg_data);
        assert_eq!(bus.ram, [0; RAM_SIZE]);
        assert_eq!(bus.ppu, [0; PPU_SIZE]);
        assert_eq!(bus.prg_rom, prg_data);
    }

    #[test]
    fn test_write_and_read_ram() {
        let mut bus = Bus::new(&[]);
        bus.write(0x0002, 0xAB);
        assert_eq!(bus.read(0x0002), 0xAB);
        // Test RAM mirroring
        bus.write(0x0802, 0xCD);
        assert_eq!(bus.read(0x0002), 0xCD);
    }

    #[test]
    fn test_write_and_read_ppu() {
        let mut bus = Bus::new(&[]);
        bus.write(0x2003, 0x55);
        assert_eq!(bus.read(0x2003), 0x55);
        // Test PPU mirroring
        bus.write(0x200B, 0x77);
        assert_eq!(bus.read(0x2003), 0x77);
    }

    #[test]
    #[should_panic(expected = "Attempted to write to PRG ROM")]
    fn test_write_to_prg_rom_panics() {
        let mut bus = Bus::new(&[0; PRG_SIZE]);
        bus.write(0x8000, 0xFF);
    }

    #[test]
    fn test_resolve_prg_rom_index_mirrored() {
        let prg_data = vec![0xAA; MIRRORED_PRG_SIZE];
        let bus = Bus::new(&prg_data);
        // Should wrap around for 16 KiB ROMs
        let idx = bus.resolve_prg_rom_index(0xC000);

        assert_eq!(idx, idx % MIRRORED_PRG_SIZE);
        let idx2 = bus.resolve_prg_rom_index(0x8000);
        assert_eq!(idx2, 0x0000);
    }

    #[test]
    fn test_load_prg_rom_success() {
        let mut bus = Bus::new(&[]);
        let data = vec![0x11; PRG_SIZE];
        let result = bus.load_prg_rom(&data);
        assert!(result.is_ok());
        assert_eq!(bus.prg_rom, data);
    }

    #[test]
    fn test_load_prg_rom_too_small() {
        let mut bus = Bus::new(&[]);
        let data = vec![];
        let result = bus.load_prg_rom(&data);
        assert_eq!(result, Err(Error::ErrorLoadingROMFile));
    }

    #[test]
    fn test_load_prg_rom_too_large() {
        let mut bus = Bus::new(&[]);
        let data = vec![0xFF; PRG_SIZE + 1];
        let result = bus.load_prg_rom(&data);
        assert_eq!(result, Err(Error::ErrorLoadingROMFile));
    }

    #[test]
    #[should_panic(expected = "Invalid address")]
    fn test_read_invalid_address_panics() {
        let bus = Bus::new(&[]);
        bus.read(0x5000);
    }
}
