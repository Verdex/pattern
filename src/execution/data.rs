
use crate::ir::{Symbol, SlotAccessType};

#[derive(Debug)]
pub enum Instr {
    Nop,
    Goto { instr_dest: usize },
    BranchFalse { relative_stack_address: usize, instr_dest : usize },
    MoveParameterToStack,
    StoreRefFromReturnPointer { dest : usize },
    StoreRefFromStack { src : usize, dest : usize },
    StoreFunPointer { src : usize, dest : usize },
    // After these instructions the VM needs to populate the rp
    Return(usize),
    ConsNumber(i64),
    ConsBool(bool),
    CallFun(usize),
    CallFunRefOnStack(usize), 
    StackSlotAccess { src: usize, slot : SlotAccessType },
}

#[derive(Debug)]
pub enum Ref {
    Heap { address: usize, offset : SlotAccessType },
    Fun { fun_address : usize, environment_address : Option<usize> },
}

#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Number(i64),
    Compound { tag : Symbol, params: Vec<Data> },
    Reference(Ref),
}