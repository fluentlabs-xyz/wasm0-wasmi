#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct InstrMeta(pub usize, pub u8);

impl InstrMeta {
    pub fn source_pc(&self) -> u32 {
        self.0 as u32
    }

    pub fn opcode(&self) -> u8 {
        self.1
    }
}
