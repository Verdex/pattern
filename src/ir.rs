
#[derive(Debug)]
pub struct ConsTag {
    pub name : String, 
}

#[derive(Debug)] 
pub struct Row { 
    pub name : String,
    pub t : Type,
 }

#[derive(Debug)]
pub enum Type {
    Infer, // TODO can this be removed?
    Generic(String),
    Concrete(String),
    Fun { input : Vec<Type>, output : Box<Type> },
    Index { name : String, params : Vec<Type> },
    Anon(Vec<Row>),
}