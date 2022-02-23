
use std::collections::HashMap;

use crate::ast;

use crate::ir::ConsTag;

use super::data::{ StaticError
                 , ConsInfo
                 , RowType
                 , ConcreteType
                 , Type
                 };


pub fn ast_to_ir_type(t : ast::Type) -> Type {

    fn m(ts : Vec<ast::Type>) -> Vec<super::data::Type> {
        ts.into_iter().map(ast_to_ir_type).collect()
    }

    let array = ConcreteType("Array".to_string());

    match t {
        ast::Type::Generic(name) => super::data::Type::Generic(name),
        ast::Type::Concrete(name) => super::data::Type::Concrete(ConcreteType(name)),
        ast::Type::Array(t) => super::data::Type::Index { name: array.clone(), params: vec![ast_to_ir_type(*t)] },
        ast::Type::Fun { input, output } => super::data::Type::Fun { input: m(input), output: Box::new(ast_to_ir_type(*output)) },
        ast::Type::Index { name, params } => super::data::Type::Index { name: ConcreteType(name), params: m(params) },
    }
}

pub fn determine_type_info( data_defs : Vec<ast::Ast> ) 
    -> Result<( HashMap<ConsTag, ConcreteType>
       , HashMap<ConcreteType, Vec<ConsInfo>> ), StaticError> {

    let mut type_lookup = HashMap::new();
    let mut cons_lookup = HashMap::new();

    for data_def in data_defs {
        let (concrete_type, cons_defs) = match data_def { 
            ast::Ast::DataDef { name, cons_defs } => (ConcreteType(name), cons_defs),
            _ => panic!( "Encountered non DataDef variant"),
        };

        if cons_lookup.contains_key(&concrete_type) {
            let ConcreteType(ct) = concrete_type;
            return Err(StaticError::Fatal(format!("Encountered duplicate type name {ct}")));
        }

        let cons_infos : Vec<ConsInfo> 
            = cons_defs.into_iter()
                       .map(|c| ConsInfo{ tag: ConsTag(c.name)
                                        , ts: c.params.into_iter().map(ast_to_ir_type).collect()
                                        } ).collect();


        cons_lookup.insert(concrete_type.clone(), cons_infos.clone());

        for info in cons_infos {
            if type_lookup.contains_key(&info.tag) {
                let ConsTag(tag) = info.tag;
                return Err(StaticError::Fatal(format!("Encountered duplicate constructor name {tag}")));
            }

            type_lookup.insert( info.tag, concrete_type.clone() );
        }
    }
    
    Ok((type_lookup, cons_lookup))
}