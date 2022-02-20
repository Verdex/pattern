
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

pub struct DefaultSystemCalls { }

impl SystemCalls for DefaultSystemCalls {
    fn print(&mut self, s : String) {
        println!("{}", s);
    }
}

impl VM {
    pub fn new(instructions : Vec<Instruction>, entry_point : InstructionAddress) -> Self {
        VM { instruction_pointer: entry_point 
           , instructions
           , heap: vec![]
           , outgoing_params: vec![]
           , frames: vec![]
           , current_frame: Frame { stack: vec![], return_address: InstructionAddress(0) } 
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
                                          , return_address: self.instruction_pointer.next()
                                          };

                    mem::swap(&mut frame, &mut self.current_frame);

                    self.frames.push(frame);
                    self.instruction_pointer = *address;
                    continue;
                },
                Instruction::CallFromHeap(stack_offset) => {
                    let s = get_stack(&self.current_frame.stack, *stack_offset);
                    let h = get_heap(&self.heap, s);
                    let address = match h {
                        Data::Fun(x) => x,
                        _ => panic!("CallFromHeap must call a function instruction"),
                    };

                    let incoming_params = mem::take(&mut self.outgoing_params);

                    let mut frame = Frame { stack: incoming_params 
                                          , return_address: self.instruction_pointer.next()
                                          };

                    mem::swap(&mut frame, &mut self.current_frame);

                    self.frames.push(frame);
                    self.instruction_pointer = *address;
                    continue;
                },
                Instruction::PushReturnPointerToStack => {
                    self.current_frame.stack.push(self.return_pointer);
                },
                Instruction::PushStackToParam(stack_offset) => {
                    let v = get_stack(&self.current_frame.stack, *stack_offset);
                    self.outgoing_params.push(v);
                },
                Instruction::Exit => { break; },

                // Needs to put a HeapAddress on the return_pointer
                Instruction::Multiply(offset_a, offset_b) => {
                    let a = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Number(a * b));
                    self.return_pointer = address;
                },
                Instruction::Division(offset_a, offset_b) => {
                    let a = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Number(a / b));
                    self.return_pointer = address;
                },
                Instruction::Remainder(offset_a, offset_b) => {
                    let a = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Number(a % b));
                    self.return_pointer = address;
                },
                Instruction::Addition(offset_a, offset_b) => {
                    let a = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Number(a + b));
                    self.return_pointer = address;
                },
                Instruction::Substract(offset_a, offset_b) => {
                    let a = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Number(a - b));
                    self.return_pointer = address;
                },
                Instruction::LogicalXor(offset_a, offset_b) => {
                    let a = get_heap_bool_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_bool_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Bool(a ^ b));
                    self.return_pointer = address;
                },
                Instruction::LogicalNot(offset) => {
                    let a = get_heap_bool_from_stack(&self.current_frame.stack, &self.heap, *offset);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Bool(!a));
                    self.return_pointer = address;
                },
                Instruction::LogicalOr(offset_a, offset_b) => {
                    let a = get_heap_bool_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_bool_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Bool(a || b));
                    self.return_pointer = address;
                },
                Instruction::LogicalAnd(offset_a, offset_b) => {
                    let a = get_heap_bool_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_bool_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Bool(a && b));
                    self.return_pointer = address;
                },
                Instruction::GreaterThan(offset_a, offset_b) => {
                    let a = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Bool(a > b));
                    self.return_pointer = address;
                },
                Instruction::LessThan(offset_a, offset_b) => {
                    let a = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_a);
                    let b = get_heap_number_from_stack(&self.current_frame.stack, &self.heap, *offset_b);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Bool(a < b));
                    self.return_pointer = address;
                },
                Instruction::Equal(offset_a, offset_b) => {
                    let a_r = get_stack(&self.current_frame.stack, *offset_a);
                    let a = get_heap(&self.heap, a_r);

                    let b_r = get_stack(&self.current_frame.stack, *offset_b);
                    let b = get_heap(&self.heap, b_r);

                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Bool(a == b));
                    self.return_pointer = address;
                },
                Instruction::ConsBool(b) => {
                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Bool(*b));
                    self.return_pointer = address;
                },
                Instruction::ConsNumber(n) => {
                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Number(*n));
                    self.return_pointer = address;
                },
                Instruction::ConsString(s) => {
                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::String(s.clone()));
                    self.return_pointer = address;
                },
                Instruction::ConsFunAddress(instr_addr) => {
                    let address = HeapAddress(self.heap.len());
                    self.heap.push(Data::Fun(*instr_addr));
                    self.return_pointer = address;
                },
                Instruction::ConsRef(stack_offset) => {
                    let address = HeapAddress(self.heap.len());
                    let r = get_stack(&self.current_frame.stack, *stack_offset);
                    self.heap.push(Data::Ref(r));
                    self.return_pointer = address;
                },
                Instruction::Deref(stack_offset) => {
                    let s = get_stack(&self.current_frame.stack, *stack_offset);
                    let h1 = get_heap(&self.heap, s);
                    let h2 = match h1 {
                        Data::Ref(h) => h,
                        _ => panic!("Cannot dereference a non-ref value"),
                    };
                    self.return_pointer = *h2;
                },
                Instruction::Return(stack_offset) => {
                    let r = get_stack(&self.current_frame.stack, *stack_offset);
                    self.return_pointer = r;
                    let mut prev_frame = self.frames.pop().expect("There must be a previous frame on Return");
                    self.instruction_pointer = self.current_frame.return_address;
                    mem::swap(&mut self.current_frame, &mut prev_frame);
                    continue;
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

fn get_heap_number_from_stack(stack : &Vec<HeapAddress>, heap : &Vec<Data>, offset : StackOffset) -> i64 {
    let r = get_stack(stack, offset);
    let v = get_heap(heap, r);
    match v {
        Data::Number(i) => *i,
        _ => panic!("Getting Number from heap must be number"),
    }
}

fn get_heap_bool_from_stack(stack : &Vec<HeapAddress>, heap : &Vec<Data>, offset : StackOffset) -> bool {
    let r = get_stack(stack, offset);
    let v = get_heap(heap, r);
    match v {
        Data::Bool(b) => *b,
        _ => panic!("Getting Bool from heap must be bool"),
    }
}

fn display(d : &Data) -> String {
    match d {
        Data::Bool(true) => "true".to_string(),
        Data::Bool(false) => "false".to_string(),
        Data::Number(i) => i.to_string(),
        Data::String(s) => s.to_string(),
        Data::Fun(address) => format!("function at:  {:X}", address.0),
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
    fn cons_bool_should_leave_bool_on_heap() {
        let mut sys = TestSysCall { prints: vec![] };
        let mut vm = VM::new( vec![ Instruction::ConsBool(true)
                                  , Instruction::Exit
                                  ]
                            , InstructionAddress(0));

        vm.run(&mut sys);

        let v = get_heap(&vm.heap, vm.return_pointer);

        assert!( matches!( v, Data::Bool(true) ) );
    }

    #[test]
    fn run_should_print() {
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

    #[test]
    fn call_should_setup_internals_correctly() {
        let mut sys = TestSysCall { prints: vec![] };
        let mut vm = VM::new( vec![ Instruction::Call(InstructionAddress(2))
                                  , Instruction::Exit
                                  , Instruction::Call(InstructionAddress(4))
                                  , Instruction::Exit
                                  , Instruction::Exit
                                  ]
                            , InstructionAddress(0));

        vm.run(&mut sys);

        assert_eq!(vm.frames.len(), 2);
        assert_eq!(vm.frames[0].return_address.0, 0);
        assert_eq!(vm.frames[1].return_address.0, 1);
        assert_eq!(vm.current_frame.return_address.0, 3);
    }

    #[test]
    fn should_return() {
        let mut sys = TestSysCall { prints: vec![] };
        let mut vm = VM::new( vec![ Instruction::Call(InstructionAddress(10))
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::Call(InstructionAddress(13))
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::Call(InstructionAddress(10))
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::Print(StackOffset(0))
                                  , Instruction::Print(StackOffset(1))
                                  , Instruction::Print(StackOffset(2))
                                  , Instruction::Exit

                                  , Instruction::ConsBool(false)
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::Return(StackOffset(0))

                                  , Instruction::ConsBool(true)
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::Return(StackOffset(0))
                                  ]
                            , InstructionAddress(0));

        vm.run(&mut sys);

        assert_eq!( sys.prints.len(), 3 );
        assert_eq!( sys.prints[0], "false" );
        assert_eq!( sys.prints[1], "true" );
        assert_eq!( sys.prints[2], "false" );
    }

    #[test]
    fn should_ref_and_deref() {
        let mut sys = TestSysCall { prints: vec![] };
        let mut vm = VM::new( vec![ Instruction::ConsBool(true)
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::ConsRef(StackOffset(0))
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::Deref(StackOffset(1))
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::Print(StackOffset(2))
                                  , Instruction::Exit
                                  ]
                            , InstructionAddress(0));

        vm.run(&mut sys);

        assert_eq!( sys.prints.len(), 1 );
        assert_eq!( sys.prints[0], "true" );
    }

    #[test]
    fn should_use_params() {
        let mut sys = TestSysCall { prints: vec![] };
        let mut vm = VM::new( vec![ Instruction::ConsBool(true)
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::ConsBool(false)
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::PushStackToParam(StackOffset(0))
                                  , Instruction::PushStackToParam(StackOffset(1))
                                  , Instruction::Call(InstructionAddress(8))
                                  , Instruction::Exit

                                  , Instruction::Print(StackOffset(0))
                                  , Instruction::Print(StackOffset(1))
                                  , Instruction::Return(StackOffset(0))
                                  ]
                            , InstructionAddress(0));

        vm.run(&mut sys);

        assert_eq!( sys.prints.len(), 2 );
        assert_eq!( sys.prints[0], "true" );
        assert_eq!( sys.prints[1], "false" );
    }

    #[test]
    fn should_call_from_heap() {
        let mut sys = TestSysCall { prints: vec![] };
        let mut vm = VM::new( vec![ Instruction::Print(StackOffset(0)) 
                                  , Instruction::Return(StackOffset(0))

                                  , Instruction::ConsBool(true)
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::PushStackToParam(StackOffset(0))
                                  , Instruction::ConsFunAddress(InstructionAddress(0))
                                  , Instruction::PushReturnPointerToStack
                                  , Instruction::CallFromHeap(StackOffset(1))
                                  , Instruction::Exit
                                  ]
                            , InstructionAddress(2));

        vm.run(&mut sys);
        assert_eq!( sys.prints.len(), 1 );
        assert_eq!( sys.prints[0], "true" );
    }
}