use crate::cpu::traits::CPUOperations;

type Instruction = Box<dyn Fn(&mut dyn CPUOperations)>;

pub fn build_op_table() -> [Instruction; 256] {

    // Define a macro to wrap trait calls into closures
    macro_rules! wrap {
        ($method:ident) => {
            Box::new(|cpu: &mut dyn CPUOperations| cpu.$method())
        };
    }

    let mut table: [Instruction; 256] = [Box::new(|cpu: &mut dyn CPUOperations| cpu.nop()) as Instruction; 256]; // default everything to `nop`

    table[0x00] = wrap!(brk);
    table[0x10] = wrap!(bpl);
    table[0x20] = wrap!(jsr);
    table[0x30] = wrap!(bmi);
    table[0x40] = wrap!(rti);
    table[0x50] = wrap!(bvc);
    table[0x60] = wrap!(rts);
    table[0x70] = wrap!(bvs);
    table[0xEA] = wrap!(nop);
    table[0x90] = wrap!(bcc);
    table[0xA0] = wrap!(ldy);
    table[0xB0] = wrap!(bcs);
    table[0xC0] = wrap!(cpy);
    table[0xD0] = wrap!(bne);
    table[0xE0] = wrap!(cpx);
    table[0xF0] = wrap!(beq);

    table
}
