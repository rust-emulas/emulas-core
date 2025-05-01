mod memory;

use memory::bus::Bus;

fn main() {
    let mut bus = Bus::new();

    // Simular a escrita do opcode `LDA #0x42` (0xA9) em 0x0000 e o operando 0x42 em 0x0001
    let addr = 0x0000;

    bus.write(addr, 0xA9);
    bus.write(addr + 1, 1);

    // Verificar se foi escrito corretamente
    let op = bus.read(addr);
    let val = bus.read(addr + 1);

    println!("Opcode na RAM: {:#X}", op);
    println!("Operando na RAM: {:#X}", val);
}
