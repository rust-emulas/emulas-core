use crate::sys::interfaces::ROMFs;

use std::fs::File;
use std::io::Read;

pub struct ROMFileLinux {
    pub rom_path: String,
    pub content: Vec<u8>,
    pub size: usize,
}

impl ROMFs for ROMFileLinux {
    fn new(rom_path: String) -> Self {
        let mut rom = ROMFileLinux {
            rom_path: rom_path.clone(),
            content: Vec::new(),
            size: 0,
        };

        rom.content = Self::read_file(&rom_path);
        rom.size = rom.content.len();

        rom
    }

    fn read_file(rom_path: &str) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut f = File::open(rom_path).expect("Erro when tried to open ROM file!");

        match f.read_to_end(&mut buffer) {
            Ok(_) => buffer.clone(),
            Err(_) => panic!("Erro when tried to read ROM file!"),
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

