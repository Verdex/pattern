
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::ast::Ast;
use crate::ir::{Ir, Symbol};

use super::data::StaticError;
use super::type_info;

static SYM_GEN_COUNT : AtomicUsize = AtomicUsize::new(0);

// TODO every symbol from the ast needs to go through this
fn sym_gen(base : &str) -> Symbol {
    let post_fix = SYM_GEN_COUNT.fetch_add(1, Ordering::Relaxed);

    Symbol(format!("sym_gen_{base}_{post_fix}"))
}

pub fn generate( asts : Vec<Ast> ) -> Result<Vec<Ir>, StaticError> {

    let (datas, funcs) : (Vec<Ast>, Vec<Ast>) = asts.into_iter().partition(|tl| match tl {
        Ast::DataDef { .. } => true,
        Ast::FunDef { .. } => false,
    });

    let (tag_to_type, type_to_info) = type_info::determine_type_info(datas)?;

    Ok(vec![])
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