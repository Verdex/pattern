
use super::instr::InstructionAddress;

#[derive(Debug, Clone, Copy)]
pub struct HeapAddress(pub usize);


#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Number(i64),
    String(String),
    Fun(InstructionAddress),
    Ref(HeapAddress),
}

#[derive(Debug)]
pub struct Frame {
    pub return_address : InstructionAddress,
    pub stack : Vec<HeapAddress>,
}