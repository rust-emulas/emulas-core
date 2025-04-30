mod memory;

use memory::bus::NesBus;

fn main() {
    let mut bus = NesBus::new();

    // Simular a escrita do opcode `LDA #0x42` (0xA9) em 0x0000 e o operando 0x42 em 0x0001
    let addr = 0x0000;

    bus.write(addr, 0xA9); // opcode LDA #imediato
    bus.write(addr + 1, 0x42); // valor imediato (0x42)
    bus.write(0x8000, 0xA9);
    bus.write(0x8001, 32);

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
