
pub fn run(){
    //pretest
    let x = five();
    println!("{}", x);

    //basic function with statements and expressions
    this_is_statement();
    let b = this_is_expression();
    println!("This is expression: {}", b);

    print_number(234);

    let sum = add_without_semicolon(23, 42);
    println!("Sum: {}", sum);

    let sum2 = add_with_semicolon(50, 100);
    println!("Sum2: {}", sum2);
}


fn five() -> i32 {
    5
}

//wrong expression, since on last line we have semicolon, it will not return a value, it will return unit type ()
// fn five() -> i32 {
//     5;
// }

//this is statement, it does not produce a value, it ends with a semicolon
fn this_is_statement(){
    println!("This is a statement, it does not produce a value");
}

// this is expression, produces a value, it does not end with a semicolon
fn this_is_expression() -> char {

    'A'
}

//function with parameters, parameter should be declared with type, since Rust prioritizes Clarity + Compile-time safety, 
//it requires us to specify the type of parameters in function definition. 
//This allows the compiler to catch type errors at compile time and helps ensure that our code is correct and safe.
fn print_number(x: i32) {
    println!("Number: {}", x);
}

//function returning value
//no return, no semicolon
//Rust prefer this
fn add_without_semicolon(x: i32, y: i32) -> i32 {
    x + y
}

//function returning value
//no return, with semicolon
fn add_with_semicolon(x: i32, y: i32) -> i32 {
    return x + y;
}