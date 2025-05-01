const RAM_SIZE: usize = 2048; // 2 KiB = 0x07FF - 0x0000 + 1
const PGROM_SIZE: usize = 32768; // 32 KiB = 0xFFFF - 0x8000 + 1
const PPU_SIZE: usize = 8192; // 8 KiB = 0x3FFF - 0x2000 + 1

pub struct Bus {
    ram: [u8; RAM_SIZE],
    pgrom: [u8; PGROM_SIZE], // 32 KiB = 0xFFFF - 0x8000 + 1
    pgrom_mirror_size: usize,
    ppuram: [u8; PPU_SIZE], // 8 KiB = 0x2000 - 0x3FFF + 1
    ppuram_mirror_size: usize,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; RAM_SIZE],
            pgrom: [0; PGROM_SIZE],
            pgrom_mirror_size: 0x8000,
            ppuram: [0; PPU_SIZE],
            ppuram_mirror_size: 0x2000,
        }
    }

    #[inline(always)]
    fn mirror_index(addr: u16, base: u16, size: usize, rom_size: usize) -> usize {
        let offset = addr - base;
        if rom_size == size {
            (offset % size as u16) as usize
        } else {
            offset as usize
        }
    }

    #[inline(always)]
    const fn mirror_ram(addr: u16) -> usize {
        (addr & 0x07FF) as usize
    }

    #[inline(always)]
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[Self::mirror_ram(addr)], // RAM + mirrors
            0x2000..=0x3FFF => {
                let idx = Self::mirror_index(addr, 0x2000, 0x2000, self.ppuram_mirror_size);
                self.ppuram[idx]
            }
            0x8000..=0xFFFF => {
                let idx = Self::mirror_index(addr, 0x8000, 0x4000, self.pgrom_mirror_size);
                self.pgrom[idx]
            }
            _ => {
                eprintln!("Tentativa de acesso inválido: {:#X}", addr);
                0xFF
            }
        }
    }

    #[inline(always)]
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram[Self::mirror_ram(addr)] = value,
            0x2000..=0x3FFF => {
                let idx = Self::mirror_index(addr, 0x2000, 0x2000, self.ppuram_mirror_size);
                self.ppuram[idx] = value
            }
            0x8000..=0xFFFF => {
                let idx = Self::mirror_index(addr, 0x8000, 0x4000, self.pgrom_mirror_size);
                self.pgrom[idx] = value;
            }
            _ => {
                eprintln!("Tentativa de acesso inválido: {:#X}", addr);
            }
        }
    }
}
