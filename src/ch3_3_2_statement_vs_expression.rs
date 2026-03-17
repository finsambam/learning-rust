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

    if_expression();

    exercise_2();

    println!("excercise_3: {}", f({
        let y = 1;
        y + 1
    }));
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

fn if_expression() {
    let number = 8;
    let result = if number > 5 {
        "big"
    } else {
        "small"
    };

    println!("The number is: {}", result);
}

fn exercise_2() {
    let x = 10;

    let result = if x % 2 == 0 {
        x * 2
    } else {
        x * 3
    };

    println!("{}", result);
}

fn f(x: i32) -> i32 { x + 1 }
