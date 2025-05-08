use super::interpreter::Interpreter;
use core::panic;

pub type OpcodeFn = fn(&mut Interpreter, u16);

macro_rules! opcode_table {
    (
        $(
            $key:literal $(..= $end:literal)? => $func:expr
        ),* $(,)?
    ) => {{
        const fn build() -> [OpcodeFn; 256] {
            let mut table = [invalid_opcode as OpcodeFn; 256];

            $(
                opcode_table!(@expand table, $key $(, $end)?, $func);
            )*

            table
        }

        build()
    }};

    // Range
    (@expand $table:ident, $start:literal, $end:literal, $func:expr) => {{
        let mut i = $start;
        while i <= $end {
            $table[i] = $func;
            i += 1;
        }
    }};

    // Único literal
    (@expand $table:ident, $key:literal, $func:expr) => {{
        $table[$key] = $func;
    }};
}

// TODO: Implement the actual opcodes here
fn invalid_opcode(i: &mut Interpreter, addr: u16) {
    i.memory.write(addr, 0x0);
    panic!("Invalid opcode executed!");
}

pub static OPCODES: [OpcodeFn; 256] = opcode_table! {
    // 0x00..=0x1F
    0x00..=0x1F => invalid_opcode,
};
