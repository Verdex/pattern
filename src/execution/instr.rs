
#[derive(Debug, Clone, Copy)]
pub struct StackOffset(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct InstructionAddress(pub usize);

impl InstructionAddress {
    pub fn next(&self) -> InstructionAddress {
        InstructionAddress(self.0 + 1)
    }
}

#[derive(Debug)]
pub enum Instruction { 
   Print(StackOffset),
   Call(InstructionAddress),
   Exit,
}