
#[derive(Debug)]
pub enum Ast {

}

#[derive(Debug)]
pub struct FunParam {
    pub name : String,
    pub t : Option<Type>,
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Variable(String),
    Cons { name : String, params : Vec<Expr> },
    Let { name : String, t : Option<Type>, value : Box<Expr>, expr : Box<Expr> },
    Lambda { params : Vec<FunParam>, return_type : Option<Type>, expr : Box<Expr> },
    Array(Vec<Expr>),
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
pub enum StandardPattern {

}

#[derive(Debug)]
pub enum ArrayPattern {

}

#[derive(Debug)]
pub enum PathPattern {

}