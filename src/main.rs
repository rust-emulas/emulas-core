mod memory;

use memory::bus::Bus;

fn main() {
    let mut bus = Bus::new();

    // Simular a escrita do opcode `LDA #0x42` (0xA9) em 0x0000 e o operando 0x42 em 0x0001
    let addr = 0x0000;

    // Verificar se foi escrito corretamente
    let op = bus.read(addr);
    let val = bus.read(addr + 1);

    println!("Opcode na RAM: {:#X}", op);
    println!("Operando na RAM: {:#X}", val);

    let op = bus.read(0x8000);
    let val = bus.read(0x8000 + 1);

    println!("Opcode na RAM: {:#X}", op);
    println!("Operando na RAM: {:#X}", val);
}
