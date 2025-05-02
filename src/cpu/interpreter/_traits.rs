
pub trait CPUInterface {
    fn new() -> Self;
    fn reset(&mut self);
    fn fetch(&mut self, instruction: Vec<u8>) -> u8;
}

pub trait CPUOperations {
    fn brk(&mut self);
    fn bpl(&mut self);
    fn jsr(&mut self);
    fn bmi(&mut self);
    fn rti(&mut self);
    fn bvc(&mut self);
    fn rts(&mut self);
    fn bvs(&mut self);
    fn nop(&mut self);
    fn bcc(&mut self);
    fn ldy(&mut self);
    fn bcs(&mut self);
    fn cpy(&mut self);
    fn bne(&mut self);
    fn cpx(&mut self);
    fn beq(&mut self);
}