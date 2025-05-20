pub mod flags;
pub mod instruction;

use flags::StatusFlags;
use instruction::AddressingMode;

use crate::memory::Bus;

pub struct Cpu {
    pub pc: u16,             // Program Counter
    pub sp: u8,              // Stack Pointer
    pub a: u8,               // Accumulator
    pub x: u8,               // X Register
    pub y: u8,               // Y Register
    pub status: StatusFlags, // Status Register
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0xFFFD,
            sp: 0xFF,
            a: 0x00,
            x: 0x00,
            y: 0x00,
            status: StatusFlags::empty(),
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0xFFFD;
        self.sp = 0xFF;
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.status = StatusFlags::empty();
    }

    // Set the accumulator register and update the status flags accordingly
    pub fn set_a(&mut self, value: u8) {
        self.a = self.status.set_register_flags(value);
    }

    // Set the x register and update the status flags accordingly
    pub fn set_x(&mut self, value: u8) {
        self.x = self.status.set_register_flags(value);
    }

    // Set the y register and update the status flags accordingly
    pub fn set_y(&mut self, value: u8) {
        self.y = self.status.set_register_flags(value);
    }

    pub fn handle_brk(&mut self, _bus: &mut Bus) {
        println!("BRK instruction: interrupt triggered");
    }
}
