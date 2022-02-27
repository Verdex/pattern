
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

    Ok(vec![])
}

fn fun_types(funs : &Vec<Ast>) -> Result<HashMap<Symbol, Type>, StaticError> {
    let mut m = HashMap::new();
    for fun in funs {
        let (name, params, return_type) = match fun {
            Ast::FunDef { name, params, return_type, .. } => (Symbol::User(name.to_string()), params, return_type),
            _ => panic!("fun_types should not have any data defs"),
        }; 
        if m.contains_key(&name) {
            let x = match name {
                Symbol::User(n) => n,
                _ => panic!("fun_types should only have user symbols at this point"),
            };
            return Err(StaticError::Fatal(format!("Encountered already defined function {x}")));
        }
        let input = params.iter()
                          .map(|p| p.t.as_ref().expect("FunDef must have type on each param"))
                          .map(type_info::ast_to_ir_type)
                          .collect();

        m.insert(name, Type::Fun { input, output: Box::new(type_info::ast_to_ir_type(return_type.clone()))});
    }
    Ok(m)
}


#[cfg(test)]
mod test {
    use super::*;

}