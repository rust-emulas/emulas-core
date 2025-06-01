use crate::memory::Bus;

use super::Cpu;

#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionKind {
    BRK,
}

pub struct Instruction {
    pub opcode: u8,
    pub name: &'static str,
    pub mode: AddressingMode,
    pub kind: InstructionKind,
    pub cycles: u8,
    pub bytes: u8,
}

impl InstructionKind {
    pub fn execute(&self, mode: AddressingMode, cpu: &mut Cpu, bus: &mut Bus) {
        match self {
            InstructionKind::BRK => cpu.handle_brk(bus),
        }
    }
}

// Exemplo de uso
pub static INSTRUCTION_TABLE: [Instruction; 1] = [Instruction {
    opcode: 0x00,
    name: "BRK",
    mode: AddressingMode::Implied,
    kind: InstructionKind::BRK,
    cycles: 7,
    bytes: 1,
}];
