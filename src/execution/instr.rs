
#[derive(Debug, Clone, Copy)]
pub struct StackOffset(pub usize)

#[derive(Debug, Clone, Copy)]
pub struct InstructionAddress(pub usize)

#[derive(Debug)]
pub enum Instruction { 
   Print(StackOffset),
   Call(InstructionAddress),
}