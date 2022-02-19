
use std::collections::HashMap;

use crate::ir::{Ir, Statement, Expr, Symbol, SlotAccessType};

use super::data::{ Instr
                 , Data
                 , Ref
                 , Instructions
                 , Heap
                 , Stack
                 , Addressable
                 , InstructionAddress
                 , HeapAddress
                 , StackAddress
                 , Address
                 };


fn ir_to_instr( irs : Vec<Ir> ) -> (Instructions, InstructionAddress) {
    let mut instrs = Instructions::new();
    let mut symbol_to_fun_address = HashMap::new();
    let mut symbol_to_relative_stack_address = HashMap::new();
    let mut entry_point = InstructionAddress::new(0);

    for ir in irs {
        if ir.name == Symbol("main".to_string()) {
            entry_point = instrs.fresh_address();
        }

        symbol_to_fun_address.insert(ir.name, instrs.fresh_address());
        let mut relative_stack_address : usize = 0;
        for param in ir.params {
            symbol_to_relative_stack_address.insert(param, relative_stack_address);
            relative_stack_address+=1;
            instrs.push(Instr::MoveParameterToStack);
        }

        for statement in ir.statements {
            match statement {
                Statement::Assign { name, expr } => {

                    let target = relative_stack_address;
                    symbol_to_relative_stack_address.insert(name, target);
                    relative_stack_address+=1;

                    match *expr {
                        Expr::Number(v) => {
                            instrs.push(Instr::ConsNumber(v));
                            instrs.push(Instr::StoreRefFromReturnPointer { relative_stack_address: target });
                        },
                        Expr::Bool(v) => {
                            instrs.push(Instr::ConsBool(v));
                            instrs.push(Instr::StoreRefFromReturnPointer { relative_stack_address: target });
                        },
                        Expr::Variable(name) if symbol_to_relative_stack_address.contains_key(&name) => {
                            let rsa = symbol_to_relative_stack_address.get(&name).expect("Could not find relative stack address for symbol");
                            instrs.push(Instr::StoreRefFromStack { src : *rsa, dest : target });
                        },
                        Expr::Variable(name) if symbol_to_fun_address.contains_key(&name) => {
                            let rsa = symbol_to_fun_address.get(&name).expect("Could not find function address for symbol");
                            instrs.push(Instr::StoreFunPointer { src : *rsa, dest : target });
                        },
                        Expr::Variable(_) => panic!("Unknown variable symbol"),
                        Expr::SlotAccess { data, slot } => {
                            let rsa = symbol_to_relative_stack_address.get(&data).expect("Could not find relative stack address for symbol");
                            instrs.push(Instr::StackSlotAccess{ src: *rsa, slot });
                            instrs.push(Instr::StoreRefFromReturnPointer { relative_stack_address: target });
                        }, 
                        Expr::FunCall { name, params } if symbol_to_relative_stack_address.contains_key(&name) => {
                            let rsa = symbol_to_relative_stack_address.get(&name).expect("Could not find relative stack address for symbol");
                            for param in params { 
                                let p = symbol_to_relative_stack_address.get(&param).expect("Could not find relative stack address for symbol");
                                instrs.push(Instr::MoveStackToParameter{ relative_stack_address: *p });
                            }
                            instrs.push(Instr::CallFunRefOnStack { relative_stack_address: *rsa });
                            instrs.push(Instr::StoreRefFromReturnPointer { relative_stack_address: target });
                        },
                        Expr::FunCall { name, params } if symbol_to_fun_address.contains_key(&name) => {
                            let fun_instr_address = symbol_to_fun_address.get(&name).expect("Could not find function address for symbol");
                            for param in params { 
                                let p = symbol_to_relative_stack_address.get(&param).expect("Could not find relative stack address for symbol");
                                instrs.push(Instr::MoveStackToParameter{ relative_stack_address: *p });
                            }
                            instrs.push(Instr::CallFun(*fun_instr_address));
                            instrs.push(Instr::StoreRefFromReturnPointer { relative_stack_address: target });
                        },
                        Expr::FunCall { .. } => panic!("Unknown function symbol"),
                    }

                },
                Statement::Label(name) => {
                    symbol_to_fun_address.insert(name, instrs.fresh_address());
                    instrs.push(Instr::Nop);
                },
                Statement::BranchFalse { target, dest } if symbol_to_fun_address.contains_key(&dest) 
                                                        && symbol_to_relative_stack_address.contains_key(&target)
                    => {

                        let dest = symbol_to_fun_address.get(&dest).expect("Could not find function address for symbol");
                        let rsa = symbol_to_relative_stack_address.get(&target).expect("Could not find relative stack address for symbol");

                        instrs.push(Instr::BranchFalse { relative_stack_address: *rsa, instr_dest: *dest });
                },
                Statement::BranchFalse { .. } => panic!("Could not find function address or relative stack address for symbol"),
                Statement::Goto(dest) => {
                    let dest = symbol_to_fun_address.get(&dest).expect("Could not find function address for symbol");
                    instrs.push(Instr::Goto { instr_dest: *dest });
                },
                Statement::Goto(dest) => panic!("Could not find function address for symbol"),
                Statement::Return(name) => {
                    let rsa = symbol_to_relative_stack_address.get(&name).expect("Could not find relative stack address for symbol");
                    instrs.push(Instr::Return { relative_stack_address: *rsa });
                },
            }
        }
    }

    (instrs, entry_point)
}

pub fn run( ir : Vec<Ir> ) {
    let (instructions, mut ip) = ir_to_instr(ir); 

    let mut stack = Stack::new();
    let mut sp = StackAddress::new(0); 

    let mut heap = Heap::new();

    let mut rp : Ref = Ref::Fun{ fun_address: InstructionAddress::new(0), environment_address: None };

    let mut params : Vec<Ref> = vec![];

    loop {
        match instructions.get(ip) {
            Instr::Nop => { ip.inc(1); },
            Instr::Exit => { break; },
            Instr::Goto { instr_dest } => { ip = *instr_dest; },
            Instr::BranchFalse { relative_stack_address, instr_dest } => {
                let r = stack.get(sp.offset(*relative_stack_address));
                match r {
                    Ref::Heap(address) => {
                        let v = heap.get(*address);
                        match v {
                            Data::Bool(true) => { },
                            Data::Bool(false) => {
                                ip = *instr_dest;
                            },
                            _ => panic!("Branch false called on non bool value"),
                        }
                    }, 
                    Ref::Fun { .. } => panic!("Branch false called on function address"),
                }
            },
            Instr::MoveParameterToStack => {
                let p = params.remove(1);
                stack.push(p);
            },
            Instr::MoveStackToParameter { relative_stack_address } => {
                let p = stack.get(sp.offset(*relative_stack_address));
                params.push(*p);
            },
            Instr::StoreRefFromReturnPointer { relative_stack_address } => {
                stack.push(rp.clone());
            },
            Instr::StoreRefFromStack { src, dest } => {
                let s = stack.get(sp.offset(*src));
                stack.push(s.clone());
            },
            Instr::StoreFunPointer { src, dest } => {
                
            },

            // After these instructions the VM needs to populate the rp
            Instr::Return { relative_stack_address } => {
                // TODO populate RP with whatever is at the stack
            },
            Instr::ConsNumber(n) => {

            },
            Instr::ConsBool(b) => {

            },
            Instr::CallFun(address) => {

            },
            Instr::CallFunRefOnStack { relative_stack_address } => {

            }, 
            Instr::StackSlotAccess { src, slot } => {
                let index = match slot {
                    SlotAccessType::Tag => 0,
                    SlotAccessType::Index(i) => i + 1,
                };

                let r = stack.get(sp.offset(*src));

                rp = *r; 
            },
        }
    }
}