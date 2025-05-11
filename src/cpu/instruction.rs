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

pub static Instructions: [Instruction; 1] = [Instruction {
    name: "BRK",
    mode: AddressingMode::Implied,
    kind: InstructionKind::BRK,
    cycles: 7,
    bytes: 1,
    opcode: 0x00,
}];

impl InstructionKind {
    pub fn execute(&self, mode: AddressingMode, cpu: &mut Cpu, bus: &mut Bus) {
        match self {
            InstructionKind::BRK => cpu.handle_brk(bus),
        }
    }
}

// Macro para montar a tabela de instruções
macro_rules! instruction_table {
    (
        $(
            $key:literal $(..= $end:literal)? => {
                name: $name:expr,
                mode: $mode:expr,
                kind: $kind:expr,
                cycles: $cycles:expr,
                bytes: $bytes:expr
            }
        ),* $(,)?
    ) => {{
        let mut table: [Instruction; 1] = [Instruction {
            opcode: 0x00,
            name: "???",
            mode: AddressingMode::Implied,
            kind: InstructionKind::BRK,
            cycles: 0,
            bytes: 1,
        }];

        $(
            instruction_table!(@expand table, $key $(, $end)?, {
                name: $name,
                mode: $mode,
                kind: $kind,
                cycles: $cycles,
                bytes: $bytes
            });
        )*

        table
    }};

    (@expand $table:ident, $start:literal, $end:literal, {
        name: $name:expr,
        mode: $mode:expr,
        kind: $kind:expr,
        cycles: $cycles:expr,
        bytes: $bytes:expr
    }) => {{
        let mut i = $start;
        while i <= $end {
            $table[i] = Instruction {
                opcode: i,
                name: $name,
                mode: $mode,
                kind: $kind,
                cycles: $cycles,
                bytes: $bytes,
            };
            i += 1;
        }
    }};

    (@expand $table:ident, $key:literal, {
        name: $name:expr,
        mode: $mode:expr,
        kind: $kind:expr,
        cycles: $cycles:expr,
        bytes: $bytes:expr
    }) => {{
        $table[$key] = Instruction {
            opcode: $key,
            name: $name,
            mode: $mode,
            kind: $kind,
            cycles: $cycles,
            bytes: $bytes,
        };
    }};
}

// Exemplo de uso
pub static INSTRUCTION_TABLE: [Instruction; 1] = instruction_table! {
    0x00 => {
        name: "BRK",
        mode: AddressingMode::Implied,
        kind: InstructionKind::BRK,
        cycles: 7,
        bytes: 1
    },
};

// macro_rules! opcode_table {
//     (
//         $(
//             $key:literal $(..= $end:literal)? => $func:expr
//         ),* $(,)?
//     ) => {{
//         const fn build() -> [OpcodeFn; 256] {
//             let mut table = [invalid_opcode as OpcodeFn; 256];

//             $(
//                 opcode_table!(@expand table, $key $(, $end)?, $func);
//             )*

//             table
//         }

//         build()
//     }};

//     // Range
//     (@expand $table:ident, $start:literal, $end:literal, $func:expr) => {{
//         let mut i = $start;
//         while i <= $end {
//             $table[i] = $func;
//             i += 1;
//         }
//     }};

//     // Único literal
//     (@expand $table:ident, $key:literal, $func:expr) => {{
//         $table[$key] = $func;
//     }};
// }

// // TODO: Implement the actual opcodes here
// fn invalid_opcode(i: &mut Cpu, addr: u16) {
//     panic!("Invalid opcode executed!");
// }

// pub static OPCODES: [OpcodeFn; 256] = opcode_table! {
//     // 0x00..=0x1F
//     0x00..=0x1F => invalid_opcode,
// };
