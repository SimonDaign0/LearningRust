//i32 = integer 32, u32 = unsigned integer
let x: i32;
x = 42;
//Throws away the value
let _ = x;
//does not need to be the same type
let pair = ('a', 17);
pair.0;
pair.1
//OR EXPLICIT
let pair: (char, i32) = ('a', 17);
//Duple
let (some_char, some_int) = ('a', 17)
assert!(some_char, 'a') // true so no error message
assert!(some_int, 17) //  true so no error message
// Function
fn firstFunction() {
    println("Hello World!");
}
// Function that returns an Int 32 (Return type)
fn anInteger() -> i32 {
    4
}

fn anInteger() -> i32 {
    return 4;
}
// BLOCK TYPE
let x = "out";
{
    //Diffrent X
    let x = "in";
    println("{}", x);
}
println("{}", x);
// multiple statements on a single block

let x = {
    let y = 1; //first statement
    let z = 2; //Second statement
    y + z //tail
}
