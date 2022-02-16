
use std::collections::HashMap;

use crate::ir::{Ir, Statement, Expr, Symbol, SlotAccessType};

use super::data::{ Instr
                 , Data
                 , Ref
                 , Instructions
                 , Heap
                 , Stack
                 , InstructionAddress
                 , HeapAddress
                 , StackAddress
                 , Addressable
                 };


fn ir_to_instr( irs : Vec<Ir> ) -> (Vec<Instr>, usize) {
    let mut instrs = vec![];
    let mut symbol_to_fun_address = HashMap::new();
    let mut symbol_to_relative_stack_address = HashMap::new();
    let mut entry_point : usize = 0;

    for ir in irs {
        if ir.name == Symbol("main".to_string()) {
            entry_point = instrs.len();
        }

        symbol_to_fun_address.insert(ir.name, instrs.len());
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
                            instrs.push(Instr::StoreRefFromReturnPointer { dest: target });
                        },
                        Expr::Bool(v) => {
                            instrs.push(Instr::ConsBool(v));
                            instrs.push(Instr::StoreRefFromReturnPointer { dest: target });
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
                            instrs.push(Instr::StoreRefFromReturnPointer { dest: target });
                        }, 
                        Expr::FunCall { name, params } if symbol_to_relative_stack_address.contains_key(&name) => {
                            let rsa = symbol_to_relative_stack_address.get(&name).expect("Could not find relative stack address for symbol");
                            for param in params { 
                                let p = symbol_to_relative_stack_address.get(&param).expect("Could not find relative stack address for symbol");
                                instrs.push(Instr::MoveStackToParameter{ relative_stack_address: *p });
                            }
                            instrs.push(Instr::CallFunRefOnStack(*rsa));
                            instrs.push(Instr::StoreRefFromReturnPointer { dest: target });
                        },
                        Expr::FunCall { name, params } if symbol_to_fun_address.contains_key(&name) => {
                            let fun_instr_address = symbol_to_fun_address.get(&name).expect("Could not find function address for symbol");
                            for param in params { 
                                let p = symbol_to_relative_stack_address.get(&param).expect("Could not find relative stack address for symbol");
                                instrs.push(Instr::MoveStackToParameter{ relative_stack_address: *p });
                            }
                            instrs.push(Instr::CallFun(*fun_instr_address));
                            instrs.push(Instr::StoreRefFromReturnPointer { dest: target });
                        },
                        Expr::FunCall { .. } => panic!("Unknown function symbol"),
                    }

                },
                Statement::Label(name) => {
                    symbol_to_fun_address.insert(name, instrs.len());
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
                    instrs.push(Instr::Return(*rsa));
                },
            }
        }
    }

    (instrs, entry_point)
}

pub fn run( ir : Vec<Ir> ) {
    let (instructions, entry_point) : (Vec<Instr>, usize) = ir_to_instr(ir); 

    let mut ip : usize = entry_point;

    let mut stack : Vec<Ref> = vec![];
    let mut sp : usize = 0; 

    let mut heap : Vec<Data> = vec![];

    let mut rp : Ref = Ref::Fun{ fun_address: 0, environment_address: None };

    let mut params : Vec<Ref> = vec![];

    loop {
        match &instructions[ip] {
            Instr::Nop => { ip += 1; },
            Instr::Exit => { break; },
            Instr::Goto { instr_dest } => { ip = *instr_dest; },
            Instr::BranchFalse { relative_stack_address, instr_dest } => {
                let r = stack[relative_stack_address + sp];
                match r {
                    Ref::Heap { address } => {
                        let v = &heap[address];
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
                let p = stack[*relative_stack_address];
                params.push(p);
            },
            Instr::StoreRefFromReturnPointer { dest } => {

            },
            Instr::StoreRefFromStack { src, dest } => {

            },
            Instr::StoreFunPointer { src, dest } => {

            },

            // After these instructions the VM needs to populate the rp
            Instr::Return(stack_address) => {
                // TODO populate RP with whatever is at the stack
            },
            Instr::ConsNumber(n) => {

            },
            Instr::ConsBool(b) => {

            },
            Instr::CallFun(address) => {

            },
            Instr::CallFunRefOnStack(address) => {

            }, 
            Instr::StackSlotAccess { src, slot } => {
                let index = match slot {
                    SlotAccessType::Tag => 0,
                    SlotAccessType::Index(i) => i + 1,
                };

                rp = Ref::Heap { address: src + index }; 
            },
        }
    }
}