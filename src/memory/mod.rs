use crate::sys::errors::Error;
use core::panic;

pub const RAM_SIZE: usize = 2048; // 2 KiB = 0x07FF - 0x0000 + 1 = Max 8 KiB
pub const PPU_SIZE: usize = 8192; // 8 KiB = 0x3FFF - 0x2000 + 1 = Max 8 bytes
pub const MIRRORED_PRG_SIZE: usize = 16384; // 16 KiB = 0x7FFF - 0x8000 + 1
pub const PRG_SIZE: usize = 32768; // 32 KiB = 0xFFFF - 0x8000 + 1

pub trait BusInterface {
    fn resolve_prg_rom_index(&self, addr: u16) -> usize;
    fn new(prg_rom: &[u8]) -> Self;
    fn write(&mut self, addr: u16, value: u8);
    fn read(&self, addr: u16) -> u8;
    fn load_prg_rom(&mut self, data: &[u8]) -> Result<usize, Error>;
}

pub struct Bus {
    pub ram: [u8; RAM_SIZE],
    pub ppu: [u8; PPU_SIZE],
    pub prg_rom: Vec<u8>,
}

impl BusInterface for Bus {
    fn new(prg_rom: &[u8]) -> Self {
        Bus {
            ram: [0; RAM_SIZE],
            ppu: [0; PPU_SIZE],
            prg_rom: prg_rom.to_vec(),
        }
    }

    fn load_prg_rom(&mut self, data: &[u8]) -> Result<usize, Error> {
        self.prg_rom = data.to_vec();

        if self.prg_rom.len() > PRG_SIZE {
            return Err(Error::ErrorLoadingROMFile);
        }
        Ok(self.prg_rom.len())
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM (2KB + mirrors)
            0x0000..=0x1FFF => self.ram[(addr & 0x07FF) as usize] = value,

            // PPU registers (8 registers, mirrored every 8 bytes)
            0x2000..=0x3FFF => self.ppu[(addr & 0x0007) as usize] = value,

            // APU and I/O registers
            0x4000..=0x4013 | 0x4015 | 0x4017 => {
                // TODO: To be implemented
            }

            // Joypad register (0x4016)
            0x4016 => {
                // TODO: To be implemented
            }

            0x8000..=0xFFFF => panic!("Attempted to write to PRG ROM at {:#06X}", addr),

            // Open bus behavior (optional: log invalid writes)
            _ => eprintln!("Invalid write to {:#06X}", addr),
        }
    }

    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr & 0x07FF) as usize],
            0x2000..=0x3FFF => self.ppu[(addr & 0x0007) as usize],
            0x8000..=0xFFFF => {
                let index = self.resolve_prg_rom_index(addr);
                self.prg_rom[index]
            }
            _ => panic!("Invalid address: {:#X}", addr),
        }
    }

    fn resolve_prg_rom_index(&self, addr: u16) -> usize {
        let offset = (addr - 0x8000) as usize;
        if self.prg_rom.len() == MIRRORED_PRG_SIZE {
            offset % MIRRORED_PRG_SIZE
        } else {
            offset
        }
    }
}
