
use crate::ast::Ast;
use crate::ir::Ir;

use super::data::StaticError;
use super::type_info;

pub fn generate( asts : Vec<Ast> ) -> Result<Vec<Ir>, StaticError> {
    Ok(vec![])
}