
use std::collections::HashMap;

use crate::ast;
use crate::ir;

fn determine_type_info( ast : &Vec<ast::Ast> ) 
    -> ( HashMap<ir::ConsTag, ir::ConcreteType>
       , HashMap<ir::ConcreteType, Vec<ir::ConsInfo>>) {

    let (datas, funcs) : (Vec<&ast::Ast>, Vec<&ast::Ast>) = ast.iter().partition(|tl| match tl {
        ast::Ast::DataDef { .. } => true,
        ast::Ast::FunDef { .. } => false,
        _ => panic!("Unknown top level item"),
    });

    //let concrete_types = None; // TODO datas
    (HashMap::new(), HashMap::new())
}