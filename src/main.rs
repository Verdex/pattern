
mod ast;
mod parsing;

/* TODO buildin functions:

    print(e) -> e
    eq(e, e)
    lt(number, number)
    gt(number, number)
    lte(n, n)
    gte(n, n)
    add(n, n)
    sub(n, n)
    div(n, n)
    rem(n, n)
    mul(n, n)
    not(bool)
    and(bool, bool)
    or(bool, bool)
    xor(bool, bool)
    match_all(pattern, array_expr)
    parse_all(pattern, array_expr)
    path(pattern, e)
    // Probably want all list functions to take the list as their first parameter so that
    // the dot operator makes chaining easy
    fold([a], a, fun(a, b) -> b) -> b
    filter([a], fun(a) -> bool) -> [a]   
    map([a], fun(a) -> b) -> [b]
    flatten([[a]]) -> [a]
    zip([a], [b], fun(a, b) -> c) -> [c]
    range(number, number) -> [number]
    nth : [a] -> Number -> a

    
    anon types exist but are not parsable (atm)
    path_pattern<anon>
    array_pattern<anon>

*/
fn main() {
    use parsing::parser;

    // TODO top level data is going to need to be converted into the following structures:
    // DataName -> [ConsTag * [type] ] // this will be needed to determine totality for match expr
                                       // and the param type list is needed so we know what type the captured varaibles are
    // ConsTag -> DataName // This will be needed to determine the type of a ConsExpr


    /* TODO:   Types for verification will need:  
                    Infer, 
                    Generic, 
                    Concrete, 
                    function type, 
                    array type, 
                    index type, 
                    anon object type

    */


    /* TODO:  At runtime we're looking at:
                    Cons
                    Let (lexical scope for variable lookup)
                    Funcall
                    if
                    destructure
                    closure
                    data tag


    */


    let _x = parser::parse("input");
}
