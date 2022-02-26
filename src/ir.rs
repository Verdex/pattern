
#[derive(Debug)]
pub struct Ir {
    pub name : Symbol,
    pub params : Vec<Symbol>,
    pub statements : Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Assign { name : Symbol, expr : Expr },
    If { target: Symbol, statements : Vec<Statement> },
    Return(Symbol),
}

#[derive(Debug)] 
pub enum SlotAccessType {
    Tag,
    Index(usize),
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Variable(Symbol),
    Array(Vec<Expr>),
    Constructor { cons_tag : ConsTag, slots_assigns : Vec<Expr> },
    Environment(Vec<Symbol>),
    SlotAccess { data : Symbol, slot : SlotAccessType }, 
    FunCall { name : Symbol, params : Vec<Symbol> },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Name {
    User,
    System,
    Anon,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol(pub String, pub Name);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ConsTag(pub String, pub Name);