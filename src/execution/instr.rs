
#[derive(Debug, Clone, Copy)]
pub struct StackOffset(pub usize);

#[derive(Debug, Clone, Copy, PartialEq)]
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
   CallFromHeap(StackOffset),
   PushReturnPointerToStack,
   PushStackToParam(StackOffset),
   BranchFalse(StackOffset, InstructionAddress),
   Move { src : StackOffset, dest: StackOffset },
   MoveReturnPointerToStack(StackOffset),
   Exit,
   // Needs to put a HeapAddress on the return_pointer
   PopStack,
   Multiply(StackOffset, StackOffset),
   Division(StackOffset, StackOffset),
   Remainder(StackOffset, StackOffset),
   Addition(StackOffset, StackOffset),
   Substract(StackOffset, StackOffset),
   LogicalXor(StackOffset, StackOffset),
   LogicalNot(StackOffset),
   LogicalOr(StackOffset, StackOffset),
   LogicalAnd(StackOffset, StackOffset),
   GreaterThan(StackOffset, StackOffset),
   LessThan(StackOffset, StackOffset),
   Equal(StackOffset, StackOffset),
   ConsBool(bool),
   ConsNumber(i64),
   ConsString(String),
   ConsFunAddress(InstructionAddress),
   ConsRef(StackOffset),
   Deref(StackOffset),
   Return(StackOffset),
}