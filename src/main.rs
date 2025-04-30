type NesBus = Bus<2048, 0x8000>;

const fn mirror_ram(addr: u16) -> usize {
    (addr & 0x07FF) as usize
}

struct Bus<const N: usize, const M: usize> {
    ram: [u8; N],
    pgrom: [u8; M], // 32 KiB = 0xFFFF - 0x8000 + 1
}

impl<const N: usize, const M: usize> Bus<N, M> {
    fn new() -> Self {
        Bus {
            ram: [0; N],
            pgrom: [0; M],
        }
    }

    #[inline(always)]
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[mirror_ram(addr)], // RAM + mirrors
            0x8000..=0xFFFF => self.pgrom[(addr - 0x8000) as usize], // PRG-ROM
            _ => {
                eprintln!("Tentativa de acesso inválido: {:#X}", addr);
                0xFF
            }
        }
    }

    #[inline(always)]
    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram[mirror_ram(addr)] = value,
            _ => {
                eprintln!("Tentativa de acesso inválido: {:#X}", addr);
            }
        }
    }
}

fn main() {
    let mut bus = NesBus::new();

    // Simular a escrita do opcode `LDA #0x42` (0xA9) em 0x0000 e o operando 0x42 em 0x0001
    let addr = 0x0000;

    bus.write(addr, 0xA9); // opcode LDA #imediato
    bus.write(addr + 1, 0x42); // valor imediato (0x42)

    // Verificar se foi escrito corretamente
    let op = bus.read(addr);
    let val = bus.read(addr + 1);

    println!("Opcode na RAM: {:#X}", op);
    println!("Operando na RAM: {:#X}", val);
}
