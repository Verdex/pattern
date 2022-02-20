
#[derive(Debug, Clone, Copy)]
pub struct StackOffset(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct InstructionAddress(pub usize);

impl InstructionAddress {
    pub fn next(&self) -> InstructionAddress {
        InstructionAddress(self.0 + 1)
    }
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug)]
pub enum Instruction { 
   Print(StackOffset),
   Call(InstructionAddress),
   PushReturnPointerToStack,
   PushStackToParam(StackOffset),
   Exit,
   // Needs to put a HeapAddress on the return_pointer
   ConsBool(bool),
   ConsNumber(i64),
   ConsString(String),
   ConsFunAddress(InstructionAddress),
   ConsRef(StackOffset),
   Return(StackOffset),
}