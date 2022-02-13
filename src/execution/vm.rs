
use std::collections::HashMap;

use crate::ir::Ir;

use super::data::{Instr, Data, Ref};


pub fn run( ir : Vec<Ir> ) {
    let instructions : Vec<Instr> = vec![]; 
    let mut ip : usize = 0;

    let mut stack : Vec<Ref> = vec![];
    let mut sp : usize = 0; 

    let mut heap : Vec<Data> = vec![];
}