use bitflags::bitflags;

bitflags! {
    pub struct StatusFlags: u8 {
        const CARRY             = 0b0000_0001; // C
        const ZERO              = 0b0000_0010; // Z
        const INTERRUPT_DISABLE = 0b0000_0100; // I
        const DECIMAL_MODE      = 0b0000_1000; // D
        const BREAK             = 0b0001_0000; // B
        const RESERVED          = 0b0010_0000;
        const OVERFLOW          = 0b0100_0000; // V
        const NEGATIVE          = 0b1000_0000; // N
    }
}

impl StatusFlags {
    // Clear the provided flags in the status register
    pub fn clear_flags(&mut self, flags: StatusFlags) {
        self.remove(flags);
    }

    // Set the zero and negative flags based on the value
    pub fn set_zero_negative_flags(&mut self, value: u8) {
        if value == 0 {
            self.insert(StatusFlags::ZERO);
        } else if (value & 0x80) != 0 {
            self.insert(StatusFlags::NEGATIVE);
        }
    }

    // Set the zero and negative flags based on the value and return the value of the register
    pub fn set_register_flags(&mut self, value: u8) -> u8 {
        self.clear_flags(StatusFlags::ZERO | StatusFlags::NEGATIVE);
        self.set_zero_negative_flags(value);

        value
    }
}
