
pub fn run(){
    //pretest
    let x = five();
    println!("{}", x);

    //basic function with statements and expressions
    this_is_statement();
    let b = this_is_expression();
    println!("This is expression: {}", b);

    print_number(234);

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

//function with parameters
fn print_number(x: i32) {
    println!("Number: {}", x);
}