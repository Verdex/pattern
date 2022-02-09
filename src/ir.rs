
#[derive(Debug)]
pub struct ConsTag(String);

#[derive(Debug)]
pub struct ConsInfo { 
    pub tag : ConsTag,
    pub ts : Vec<Type>,
}

#[derive(Debug)] 
pub struct RowType { 
    pub name : String,
    pub t : Type,
 }

 #[derive(Debug)]
 pub struct ConcreteType(String);

#[derive(Debug)]
pub enum Type {
    Infer, // TODO can this be removed?
    Generic(String),
    Concrete(ConcreteType),
    Fun { input : Vec<Type>, output : Box<Type> },
    Index { name : String, params : Vec<Type> },
    Anon(Vec<RowType>),
}