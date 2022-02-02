
#[derive(Debug)]
pub enum Ast {
    FunDef { name : String, params : Vec<FunParam>, return_type : Type, expr : Expr },
    DataDef { name : String, cons_defs : Vec<ConsDef> },
}

#[derive(Debug)]
pub struct ConsDef {
    pub name : String,
    pub params : Vec<Type>,
}

#[derive(Debug)]
pub struct FunParam {
    pub name : String,
    pub t : Option<Type>,
}

#[derive(Debug)]
pub struct Case {
    pub pattern : StandardPattern,
    pub expr : Expr,
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Variable(String),
    Cons { name : String, params : Vec<Expr> },
    Let { name : String, t : Option<Type>, value : Box<Expr>, expr : Box<Expr> },
    Lambda { params : Vec<FunParam>, return_type : Option<Type>, expr : Box<Expr> },
    Match { expr : Box<Expr>, cases : Vec<Case> },
    FunCall { fun_expr : Box<Expr>, params : Vec<Expr> },
    Array(Vec<Expr>),
    PathPattern(Vec<PathPattern>),
    ArrayPattern(Vec<ArrayPattern>),
}

#[derive(Debug)]
pub enum Type {
    Generic(String),
    Concrete(String),
    Array(Box<Type>),
    Fun { input : Vec<Type>, output : Box<Type> },
    Index { name : String, params : Vec<Type> },
}

#[derive(Debug)]
pub enum StandardArrayPattern<P> {
    Empty,
    Array { items : Vec<P>, rest : Option<Box<P>> },
}

#[derive(Debug)]
pub enum StandardPattern {
    Number(i64),
    Bool(bool),
    Variable(String),
    Cons { name : String, params : Vec<StandardPattern> },
    At { name : String, pattern : Box<StandardPattern> },
    Wildcard,
    If { pattern : Box<StandardPattern>, predicate : Box<Expr> },
    StandardArray(StandardArrayPattern<StandardPattern>),
}

#[derive(Debug)]
pub enum ArrayPattern {
    Number(i64),
    Bool(bool),
    Variable(String),
    Cons { name : String, params : Vec<ArrayPattern> },
    At { name : String, pattern : Box<ArrayPattern> },
    Wildcard,
    WildcardZeroOrMore,
    WildcardN(Box<Expr>),
    If { pattern : Box<ArrayPattern>, predicate : Box<Expr> },
    StandardArray(StandardArrayPattern<ArrayPattern>),
}

#[derive(Debug)]
pub enum PathPattern {
    Number(i64),
    Bool(bool),
    Variable(String),
    Cons { name : String, params : Vec<PathPattern> },
    At { name : String, pattern : Box<PathPattern> },
    Wildcard,
    Next(Option<i64>),
    And { name : String, output : String },
    NextAnd { order : Option<i64>, name : String, output : String },
    If { pattern : Box<PathPattern>, predicate : Box<Expr> },
    StandardArray(StandardArrayPattern<PathPattern>),
}