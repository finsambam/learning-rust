pub fn run() {
    
    //simple if statement
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //multiple conditions with else if
    let number2 = 6;

    if number2 % 4 == 0 {
        println!("number2 is divisible by 4");
    } else if number2 % 3 == 0 {
        println!("number2 is divisible by 3");
    } else if number2 % 2 == 0 {
        println!("number2 is divisible by 2");
    } else {
        println!("number2 is not divisible by 4, 3, or 2");
    }

    //if on let statetment
    let condition = true;
    let number3 = if condition { 5 } else { 10 };

    println!("number3: {}", number3);

}
