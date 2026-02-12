pub fn run() {  
    // Variables are immutable by default, but we can make them mutable using the `mut` keyword.
    //mut only change the value of the variable, not the variable itself
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants are always immutable and must have a type annotation. They can be declared in any scope, including the global scope.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing allows us to declare a new variable with the same name as a previous variable. The new variable shadows the previous one, and we can reuse the same name for different purposes.
    //Shadowing can also change the type of the variable
    let y = 10;
    let y = y + 2;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    println!("Thank you for learning Rust with me!");

    let z = 5;
    let z = z * 2;

    {
        let mut z = z;
        z += 3;
        println!("Inner: {}", z);
    }

    println!("Outer: {}", z);
}