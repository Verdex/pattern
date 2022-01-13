
mod ast;
mod parsing;

/* TODO buildin functions:

    print(e)
    input() -> [number]
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
    fold(fun(a, b) -> b, [a]) -> b
    filter(fun(a) -> bool, [a]) -> [a]
    map(fun(a) -> b, [a]) -> [b]
    flatten([[a]]) -> [a]
    zip(fun(a, b) -> c, [a], [b]) -> [c]
    range(number, number) -> [number]

    
    anon types exist but are not parsable (atm)
    path_pattern<anon>
    array_pattern<anon>

*/
fn main() {
    use parsing::parser;

    let _x = parser::parse("input");
}
