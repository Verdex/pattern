
use crate::ir::Symbol;

#[derive(Debug)]
pub enum Instr {

}

#[derive(Debug)]
pub enum Ref {
    Heap(usize),
    Fun { fun_address : usize, environment_address : Option<usize> ),
}

#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Number(i64),
    Compound { tag : Symbol, params: Vec<Data> },
    Reference(Ref),
}