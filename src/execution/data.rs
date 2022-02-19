
use super::instr::InstructionAddress;

#[derive(Debug, Clone, Copy)]
pub struct HeapAddress(pub usize);


#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Number(i64),
    String(String),
    Func(InstructionAddress),
    Ref(HeapAddress),
}

#[derive(Debug)]
pub struct Frame {
    pub incoming_params : Vec<HeapAddress>,
    pub outgoing_params : Vec<HeapAddress>,
    pub return_address : Option<InstructionAddress>,
    pub stack : Vec<HeapAddress>,
}