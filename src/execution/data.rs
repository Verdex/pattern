
use crate::ir::{Symbol, SlotAccessType};

#[derive(Debug, Clone, Copy)]
pub struct StackAddress(usize);

#[derive(Debug, Clone, Copy)]
pub struct HeapAddress(usize);

#[derive(Debug, Clone, Copy)]
pub struct InstructionAddress(usize);

pub trait Address {
    fn new(x : usize) -> Self;
    fn inc(&mut self, x : usize);
    fn offset(&self, x : usize) -> Self;
}

impl Address for StackAddress {
    fn new(x : usize) -> Self { StackAddress(x) } 
    fn inc(&mut self, x : usize) {
        self.0 = self.0 + x;
    }
    fn offset(&self, x : usize) -> Self {
        let StackAddress(v) = self;
        StackAddress(v + x)
    }
}

impl Address for HeapAddress {
    fn new(x : usize) -> Self { HeapAddress(x) } 
    fn inc(&mut self, x : usize) {
        self.0 = self.0 + x;
    }
    fn offset(&self, x : usize) -> Self {
        let HeapAddress(v) = self;
        HeapAddress(v + x)
    }
}

impl Address for InstructionAddress {
    fn new(x : usize) -> Self { InstructionAddress(x) } 
    fn inc(&mut self, x : usize) {
        self.0 = self.0 + x;
    }
    fn offset(&self, x : usize) -> Self {
        let InstructionAddress(v) = self;
        InstructionAddress(v + x)
    }
}

pub trait Addressable<T, A> {
    fn new() -> Self;
    fn push(&mut self, x : T);
    fn get(&self, address : A) -> &T;
    fn fresh_address(&self) -> A;
}

pub struct Stack {
    inner : Vec<Ref>,
}

impl Addressable<Ref, StackAddress> for Stack {
    fn new() -> Self { Stack { inner: vec![] } }
    fn push(&mut self, x : Ref) { self.inner.push(x); }
    fn get(&self, address : StackAddress) -> &Ref {
        let StackAddress(v) = address;
        &self.inner[v] 
    }
    fn fresh_address(&self) -> StackAddress {
        StackAddress(self.inner.len())
    }
}

pub struct Heap {
    inner : Vec<Data>,
}

impl Addressable<Data, HeapAddress> for Heap {
    fn new() -> Self { Heap { inner: vec![] } }
    fn push(&mut self, x : Data) { self.inner.push(x); }
    fn get(&self, address : HeapAddress) -> &Data {
        let HeapAddress(v) = address;
        &self.inner[v] 
    }
    fn fresh_address(&self) -> HeapAddress {
        HeapAddress(self.inner.len())
    }
}

pub struct Instructions {
    inner : Vec<Instr>,
}

impl Addressable<Instr, InstructionAddress> for Instructions {
    fn new() -> Self { Instructions { inner: vec![] } }
    fn push(&mut self, x : Instr) { self.inner.push(x); }
    fn get(&self, address : InstructionAddress) -> &Instr {
        let InstructionAddress(v) = address;
        &self.inner[v] 
    }
    fn fresh_address(&self) -> InstructionAddress {
        InstructionAddress(self.inner.len())
    }
}

#[derive(Debug)]
pub enum Instr {
    Nop,
    Exit,
    Goto { instr_dest: InstructionAddress },
    BranchFalse { relative_stack_address: usize, instr_dest : InstructionAddress },
    MoveParameterToStack,
    MoveStackToParameter { relative_stack_address : usize },
    StoreRefFromReturnPointer { relative_stack_address : usize },
    StoreRefFromStack { src : usize, dest : usize},
    StoreFunPointer { src : InstructionAddress, dest : usize },
    // After these instructions the VM needs to populate the rp
    Return { relative_stack_address: usize },
    ConsNumber(i64),
    ConsBool(bool),
    CallFun(InstructionAddress),
    CallFunRefOnStack{ relative_stack_address: usize }, 
    StackSlotAccess { src: usize, slot : SlotAccessType },
}

#[derive(Debug, Clone, Copy)]
pub enum Ref {
    Heap(HeapAddress),
    Fun { fun_address : InstructionAddress, environment_address : Option<HeapAddress> },
}

#[derive(Debug)]
pub enum Data {
    Bool(bool),
    Number(i64),
    Tag(Symbol),
    Reference(Ref),
}