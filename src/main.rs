
mod ast;
mod parsing;
mod ir;
mod generation;
mod execution;

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
    use generation::generator;
    use execution::vm::{VM, DefaultSystemCalls};
    use execution::instr::InstructionAddress;

    /* TODO:  At runtime we're looking at:
                    Cons
                    Let (lexical scope for variable lookup)
                    Funcall
                    if
                    destructure
                    closure
                    data tag


    */

    // TODO:  Need to add some syntax to access anon object fields
    // TODO:  I suppose you can hijack dot syntax.  
    // {| Blah(!, a), Cons(b, c) |}.path( some_object ) => [{a,b,c}]
    // Then funcall(a, {a,b,c}) => access a


    let asts = parser::parse("input").unwrap(); // TODO handle err case
    let ir = generator::generate(asts).unwrap(); // TODO handle err case

    let mut sys_calls = DefaultSystemCalls{ };
    let mut vm = VM::new(vec![], InstructionAddress(0));

    vm.run(&mut sys_calls);

    //execution::vm::run(ir);

}
