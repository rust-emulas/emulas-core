pub type NesBus = Bus<2048, 0x8000>;
const fn mirror_ram(addr: u16) -> usize {
    (addr & 0x07FF) as usize
}

const fn mirror_prg(addr: u16) -> usize {
    (addr & 0x7FFF) as usize
}

pub struct Bus<const RAM_SIZE: usize, const PGROM_SIZE: usize> {
    ram: [u8; RAM_SIZE],
    pgrom: [u8; PGROM_SIZE], // 32 KiB = 0xFFFF - 0x8000 + 1
}

impl<const RAM_SIZE: usize, const PGROM_SIZE: usize> Bus<RAM_SIZE, PGROM_SIZE> {
    pub fn new() -> Self {
        Bus {
            ram: [0; RAM_SIZE],
            pgrom: [0; PGROM_SIZE],
        }
    }

    #[inline(always)]
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[mirror_ram(addr)], // RAM + mirrors
            0x8000..=0xFFFF => self.pgrom[mirror_prg(addr)], // PRG-ROM
            _ => {
                eprintln!("Tentativa de acesso inválido: {:#X}", addr);
                0xFF
            }
        }
    }

    #[inline(always)]
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram[mirror_ram(addr)] = value,
            0x8000..=0xFFFF => self.pgrom[mirror_prg(addr)] = value,
            _ => {
                eprintln!("Tentativa de acesso inválido: {:#X}", addr);
            }
        }
    }
}
