pub fn run(){
    //Statements and Expressions
    let x = 5; // This is a statement that declares a variable
    let y = {
        let z = 3; // This is a statement inside the block
        z + 1 // This is an expression that evaluates to 4
    }; // The block itself is an expression that evaluates to 4

    println!("Value of x: {}", x);
    println!("Value of y: {}", y);

    exercise_1();
}

//Exercise
fn exercise_1() {
    let x = {
        let a = 10;
        let b = 20;
        a + b
    };

    println!("exercise_1 result: {}", x);
}