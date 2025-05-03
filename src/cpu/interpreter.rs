use super::flags::StatusFlags;

pub struct Interpreter {
    pub pc: u16,             // Program Counter
    pub sp: u8,              // Stack Pointer
    pub a: u8,               // Accumulator
    pub x: u8,               // X Register
    pub y: u8,               // Y Register
    pub status: StatusFlags, // Status Register
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
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

    pub fn irq(&mut self) {}
}

impl Interpreter {
    // Clear the provided flags in the status register
    pub fn clear_flags(&mut self, flags: StatusFlags) {
        self.status.remove(flags);
    }

    // Set the zero and negative flags based on the value
    pub fn set_zero_negative_flags(&mut self, value: u8) {
        if value == 0 {
            self.status.insert(StatusFlags::ZERO);
        } else if (value & 0x80) != 0 {
            self.status.insert(StatusFlags::NEGATIVE);
        }
    }

    // Set the zero and negative flags based on the value and return the value of the register
    pub fn set_register_flags(&mut self, value: u8) -> u8 {
        self.clear_flags(StatusFlags::ZERO | StatusFlags::NEGATIVE);
        self.set_zero_negative_flags(value);

        value
    }

    // Set the accumulator register and update the status flags accordingly
    pub fn set_a(&mut self, value: u8) {
        self.a = self.set_register_flags(value);
    }

    // Set the x register and update the status flags accordingly
    pub fn set_x(&mut self, value: u8) {
        self.x = self.set_register_flags(value);
    }

    // Set the y register and update the status flags accordingly
    pub fn set_y(&mut self, value: u8) {
        self.y = self.set_register_flags(value);
    }
}

// TODO: Implement the instructions
impl Interpreter {
    // LDA - Load Accumulator
    // Load a value into the accumulator and set the zero and negative flags accordingly
    pub fn lda(&mut self) {}
}
