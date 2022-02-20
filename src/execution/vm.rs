
use std::collections::HashMap;
use std::mem;

use super::instr::{ Instruction
                  , InstructionAddress
                  , StackOffset
                  };

use super::data::{ Frame
                 , Data
                 , HeapAddress
                 };

pub fn run( instructions : Vec<Instruction>
          , entry_point : InstructionAddress
          , mut print : impl FnMut(String) 
          ) {
   
    let mut instruction_pointer = entry_point; 
    let mut heap : Vec<Data> = vec![];
    let mut outgoing_params : Vec<HeapAddress> = vec![];
    let mut frames : Vec<Frame> = vec![];
    let mut current_frame = Frame { stack: vec![] 
                                  , return_address: None
                                  };

    loop {
        match get_instruction(&instructions, instruction_pointer) {
            Instruction::Print(stack_offset) => { 
                let r = get_stack(&current_frame.stack, *stack_offset);
                let h = get_heap(&heap, r);
                print( display(h) );
            },
            Instruction::Call(address) => {
                let incoming_params = mem::take(&mut outgoing_params);

                let frame = Frame { stack: incoming_params 
                                  , return_address: Some(instruction_pointer.next())
                                  };

                frames.push(current_frame);
                current_frame = frame;
                instruction_pointer = instruction_pointer;
            },
            Instruction::Exit => { break; },
        }
    }
} 

fn get_instruction(instructions : &Vec<Instruction>, address : InstructionAddress) -> &Instruction {
    &instructions[address.0]
}

fn get_stack(stack : &Vec<HeapAddress>, offset : StackOffset) -> HeapAddress {
    stack[offset.0]
}

fn get_heap(heap : &Vec<Data>, address : HeapAddress) -> &Data {
    &heap[address.0]
}

fn display(d : &Data) -> String {
    match d {
        Data::Bool(true) => "true".to_string(),
        Data::Bool(false) => "true".to_string(),
        Data::Number(i) => i.to_string(),
        Data::String(s) => s.to_string(),
        Data::Func(address) => format!("function at:  {:X}", address.0),
        Data::Ref(address) => format!("data at:  {:X}", address.0),
    }
}

#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    fn should_exit() {
        run( vec![ Instruction::Exit ], InstructionAddress(0), |x| { } )
    }

    #[test]
    fn should_print() {
        // TODO
        let mut x = vec![];

        run( vec![], InstructionAddress(0), |v| { x.push(v); })
    }


}