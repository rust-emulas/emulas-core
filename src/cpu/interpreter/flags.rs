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