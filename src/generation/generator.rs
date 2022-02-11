
use crate::ast::Ast;
use crate::ir::Ir;

use super::data::StaticError;
use super::type_info;

pub fn generate( asts : Vec<Ast> ) -> Result<Vec<Ir>, StaticError> {

    let (datas, funcs) : (Vec<Ast>, Vec<Ast>) = asts.into_iter().partition(|tl| match tl {
        Ast::DataDef { .. } => true,
        Ast::FunDef { .. } => false,
    });

    let (tag_to_type, type_to_info) = type_info::determine_type_info(datas)?;

    Ok(vec![])
}