
use std::collections::HashMap;

use crate::ast;
use crate::ir;

use super::data::{StaticError, self};

    /*let (datas, funcs) : (Vec<&ast::Ast>, Vec<&ast::Ast>) = ast.iter().partition(|tl| match tl {
        ast::Ast::DataDef { .. } => true,
        ast::Ast::FunDef { .. } => false,
        _ => panic!("Unknown top level item"),
    });*/

//fn type_map(ast::)

fn determine_type_info( data_defs : &Vec<ast::Ast> ) 
    -> Result<( HashMap<ir::ConsTag, ir::ConcreteType>
       , HashMap<ir::ConcreteType, Vec<ir::ConsInfo>> ), StaticError> {

    let mut type_lookup = HashMap::new();
    let mut cons_lookup = HashMap::new();

    for data_def in data_defs {
        let (concrete_type, cons_defs) = match data_def { 
            ast::Ast::DataDef { name, cons_defs } => (ir::ConcreteType(name.clone()), cons_defs),
            _ => panic!( "Encountered non DataDef variant"),
        };

        if cons_lookup.contains_key(&concrete_type) {
            let ir::ConcreteType(ct) = concrete_type;
            return Err(StaticError::Fatal(format!("Encountered duplicate type name {ct}")));
        }

        cons_defs.iter().map(|def| )

        // TODO need all the cons tags out of cons_defs
        // TODO need all of the cons infos out of cons_defs
        // probably need copies because thy're going into two different hash maps


        //cons_lookup.insert(concrete_type, )




    }
    
    Ok((type_lookup, cons_lookup))
}