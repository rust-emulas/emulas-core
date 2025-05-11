const RAM_SIZE: usize = 2048; // 2 KiB = 0x07FF - 0x0000 + 1 = Max 8 KiB
const PPU_SIZE: usize = 8; // 8 KiB = 0x3FFF - 0x2000 + 1 = Max 8 bytes
const PGROM_SIZE: usize = 32768; // 32 KiB = 0xFFFF - 0x8000 + 1

pub struct Bus {
    ram: [u8; RAM_SIZE],
    ppu: [u8; PPU_SIZE],
    prgrom: [u8; PGROM_SIZE],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; RAM_SIZE],
            ppu: [0; PPU_SIZE],
            prgrom: [0; PGROM_SIZE],
        }
    }

    #[inline(always)]
    pub fn write(&mut self, addr: u16, value: u8) {
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

            // PRG-ROM (read-only, ignore writes)
            0x8000..=0xFFFF => (),

            // Open bus behavior (optional: log invalid writes)
            _ => eprintln!("Invalid write to {:#06X}", addr),
        }
    }

    #[inline(always)]
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr & 0x07FF) as usize],
            0x2000..=0x3FFF => self.ppu[(addr & 0x0007) as usize],
            0x8000..=0xFFFF => self.prgrom[(addr & 0x7FFF) as usize],
            _ => panic!("Invalid address: {:#X}", addr),
        }
    }
}
