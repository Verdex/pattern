
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
pub enum Symbol {
    User(String),
    Anon(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ConsTag {
    User(String),
    Anon(String),
}