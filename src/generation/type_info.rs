
use std::collections::HashMap;

use crate::ast;
use crate::ir;

use super::data::StaticError;

pub fn ast_to_ir_type(t : ast::Type) -> ir::Type {

    use ast::Type as a;
    use ir::Type as i;

    fn m(ts : Vec<ast::Type>) -> Vec<ir::Type> {
        ts.into_iter().map(ast_to_ir_type).collect()
    }

    let array = ir::ConcreteType("Array".to_string());

    match t {
        a::Generic(name) => i::Generic(name),
        a::Concrete(name) => i::Concrete(ir::ConcreteType(name)),
        a::Array(t) => i::Index { name: array.clone(), params: vec![ast_to_ir_type(*t)] },
        a::Fun { input, output } => i::Fun { input: m(input), output: Box::new(ast_to_ir_type(*output)) },
        a::Index { name, params } => i::Index { name: ir::ConcreteType(name), params: m(params) },
    }
}

pub fn determine_type_info( data_defs : Vec<ast::Ast> ) 
    -> Result<( HashMap<ir::ConsTag, ir::ConcreteType>
       , HashMap<ir::ConcreteType, Vec<ir::ConsInfo>> ), StaticError> {

    let mut type_lookup = HashMap::new();
    let mut cons_lookup = HashMap::new();

    for data_def in data_defs {
        let (concrete_type, cons_defs) = match data_def { 
            ast::Ast::DataDef { name, cons_defs } => (ir::ConcreteType(name), cons_defs),
            _ => panic!( "Encountered non DataDef variant"),
        };

        if cons_lookup.contains_key(&concrete_type) {
            let ir::ConcreteType(ct) = concrete_type;
            return Err(StaticError::Fatal(format!("Encountered duplicate type name {ct}")));
        }

        let cons_infos : Vec<ir::ConsInfo> 
            = cons_defs.into_iter()
                       .map(|c| ir::ConsInfo{ tag: ir::ConsTag(c.name)
                                            , ts: c.params.into_iter().map(ast_to_ir_type).collect()
                                            } ).collect();


        cons_lookup.insert(concrete_type.clone(), cons_infos.clone());

        for info in cons_infos {
            if type_lookup.contains_key(&info.tag) {
                let ir::ConsTag(tag) = info.tag;
                return Err(StaticError::Fatal(format!("Encountered duplicate constructor name {tag}")));
            }

            type_lookup.insert( info.tag, concrete_type.clone() );
        }
    }
    
    Ok((type_lookup, cons_lookup))
}