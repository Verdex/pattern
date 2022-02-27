
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::ast::Ast;

use crate::ir::{ Ir
               , Symbol
               , ConsTag
               , Statement
               , Expr
               };

use super::data::{ StaticError
                 , Type
                 , RowType
                 , ConcreteType
                 , ConsInfo
                 };

use super::type_info;

static SYM_GEN_COUNT : AtomicUsize = AtomicUsize::new(0);

fn anon_sym(base : &str) -> Symbol {
    let post_fix = SYM_GEN_COUNT.fetch_add(1, Ordering::Relaxed);

    Symbol::Anon(format!("sym_gen_{base}_{post_fix}"))
}

fn anon_tag(base : &str) -> ConsTag {
    let post_fix = SYM_GEN_COUNT.fetch_add(1, Ordering::Relaxed);

    ConsTag::Anon(format!("cons_gen_{base}_{post_fix}"))
}


struct T {
    tag_to_type : HashMap<ConsTag, ConcreteType>,
    type_to_info : HashMap<ConcreteType, Vec<ConsInfo>>,
    sym_to_type : HashMap<Symbol, Type>,
}


pub fn generate( asts : Vec<Ast> ) -> Result<Vec<Ir>, StaticError> {

    /* TODO : no cycles
              order resulting ir so that nothing references things that later show up
              no duplicate symbol definitions
    
    */

    let (datas, funcs) : (Vec<Ast>, Vec<Ast>) = asts.into_iter().partition(|tl| match tl {
        Ast::DataDef { .. } => true,
        Ast::FunDef { .. } => false,
    });

    let (tag_to_type, type_to_info) = type_info::determine_type_info(datas)?;

    let mut t = T { tag_to_type, type_to_info, sym_to_type: HashMap::new() };


    Ok(vec![])
}

#[cfg(test)]
mod test {
    use super::*;

}