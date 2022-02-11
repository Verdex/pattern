

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ConsTag(pub String);

#[derive(Debug, Clone)]
pub struct ConsInfo { 
    pub tag : ConsTag,
    pub ts : Vec<Type>,
}

#[derive(Debug, Clone)] 
pub struct RowType { 
    pub name : String,
    pub t : Type,
 }

 #[derive(Debug, PartialEq, Eq, Hash, Clone)]
 pub struct ConcreteType(pub String);

#[derive(Debug, Clone)]
pub enum Type {
    Infer, // TODO can this be removed?
    Generic(String),
    Concrete(ConcreteType),
    Fun { input : Vec<Type>, output : Box<Type> },
    Index { name : ConcreteType, params : Vec<Type> },
    Anon(Vec<RowType>),
}