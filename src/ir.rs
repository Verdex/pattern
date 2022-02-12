
#[derive(Debug)]
pub struct Ir {
    pub name : Symbol,
    pub params : Vec<Symbol>,
    pub statements : Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Assign { name : Symbol, expr : Box<Expr> },
    If { test : Symbol, statements : Vec<Statement> },
    Label(Symbol),
    Goto(Symbol),
    Return(Symbol),
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Variable(Symbol),
    SlotAccess { data : Symbol, slot : Symbol }, 
    FunCall { name : Symbol, params : Vec<Symbol> },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol(pub String);

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