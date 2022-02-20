
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

pub struct VM {
    instruction_pointer : InstructionAddress,
    instructions : Vec<Instruction>,
    heap : Vec<Data>,
    outgoing_params : Vec<HeapAddress>,
    frames : Vec<Frame>,
    current_frame : Frame,
    return_pointer : HeapAddress,
}

pub trait SystemCalls {
    fn print(&mut self, s : String);
}

impl VM {
    pub fn new(instructions : Vec<Instruction>, entry_point : InstructionAddress) -> Self {
        VM { instruction_pointer: entry_point 
           , instructions
           , heap: vec![]
           , outgoing_params: vec![]
           , frames: vec![]
           , current_frame: Frame { stack: vec![], return_address: None } 
           , return_pointer: HeapAddress(0)
           }
    }

    pub fn run( &mut self, sys_calls : &mut impl SystemCalls ) {

        loop {
            match get_instruction(&self.instructions, self.instruction_pointer) {
                Instruction::Print(stack_offset) => { 
                    let r = get_stack(&self.current_frame.stack, *stack_offset);
                    let h = get_heap(&self.heap, r);
                    sys_calls.print( display(h) );
                },
                Instruction::Call(address) => {
                    let incoming_params = mem::take(&mut self.outgoing_params);

                    let mut frame = Frame { stack: incoming_params 
                                          , return_address: Some(self.instruction_pointer.next())
                                          };

                    mem::swap(&mut frame, &mut self.current_frame);

                    self.frames.push(frame);
                    self.instruction_pointer = *address;
                    continue;
                },
                Instruction::PushReturnPointerToStack => {
                    self.current_frame.stack.push(self.return_pointer);
                },
                Instruction::Exit => { break; },

                // Needs to put a HeapAddress on the return_pointer
                Instruction::ConsBool(b) => {
                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Bool(*b));
                    self.return_pointer = address;
                },
            }
            
            self.instruction_pointer.inc();
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

    struct TestSysCall {
        prints : Vec<String>,
    }

    impl SystemCalls for TestSysCall {
        fn print(&mut self, s : String) {
            self.prints.push(s);
        }
    }

    #[test]
    fn run_should_exit() {
        let mut sys = TestSysCall { prints: vec![] };
        let mut vm = VM::new(vec![ Instruction::Exit ], InstructionAddress(0));

        vm.run(&mut sys);
    }

    #[test]
    fn should_print() {
        let mut sys = TestSysCall { prints: vec![] };
        let mut vm = VM::new( vec![ Instruction::ConsBool(true)
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::Print(StackOffset(0))
                                  , Instruction::Exit
                                  ]
                            , InstructionAddress(0));

        vm.run(&mut sys);

        assert_eq!( sys.prints.len(), 1 );
        assert_eq!( sys.prints[0], "true" );
    }
}