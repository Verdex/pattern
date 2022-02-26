
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

// TODO every symbol from the ast needs to go through this
fn gen_sym(base : &str) -> Symbol {
    let post_fix = SYM_GEN_COUNT.fetch_add(1, Ordering::Relaxed);

    Symbol(format!("sym_gen_{base}_{post_fix}"))
}

fn gen_tag(base : &str) -> ConsTag {
    let post_fix = SYM_GEN_COUNT.fetch_add(1, Ordering::Relaxed);

    ConsTag(format!("cons_gen_{base}_{post_fix}"))
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

fn fun_to_ir( t : &mut T, fun : Ast ) -> Result<Ir, StaticError> {
    let (name, params, return_type, expr) = match fun {
        Ast::FunDef { name, params, return_type, expr } => (name, params, return_type, expr),
        _ => panic!("fun_to_ir encountered DataDef"),
    };

    let mut local_sym : HashMap<Symbol, Type> = HashMap::new();

    for param in params {
        let sym = gen_sym(&param.name);
        let t = type_info::ast_to_ir_type(param.t.expect("function params must have type info"));
        local_sym.insert(sym, t);
    }

    expr_to_statements( t, &mut local_sym, expr );

    Err(StaticError::Fatal("blarg".to_string()))
}

fn expr_to_statements( t : &T, local_sym : &mut HashMap<Symbol, Type>, expr : crate::ast::Expr ) -> Result<Vec<Statement>, StaticError> {

    use crate::ast::Expr as E;

    match expr {
        E::Number(n) => {
            let name = gen_sym("anon_num");

            Ok( vec![ Statement::Assign { name: name.clone(), expr: Expr::Number(n) }
                    ,  Statement::Return(name)
                    ] )
        },
        E::Bool(b) => {
            let name = gen_sym("anon_bool");

            Ok( vec![ Statement::Assign { name: name.clone(), expr: Expr::Bool(b) }
                    ,  Statement::Return(name)
                    ] )
        },
        E::Cons { name, params } => {
            // TODO:  The params here might correspond to a symbol which was gen_sym-ed up in local_sym

            Err(StaticError::Fatal("blarg".to_string()))
        },
        _ => panic!("TODO")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_generate_symbol() {
        let first = sym_gen("x");
        let second = sym_gen("other");

        assert_eq!( first, Symbol("sym_gen_x_0".to_string()));
        assert_eq!( second, Symbol("sym_gen_other_1".to_string()));
    }
}