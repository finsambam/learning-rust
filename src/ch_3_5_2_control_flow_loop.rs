pub fn run() {
    //basic loop
    // loop {
    //     println!("again!");
    // }
    

    //return value from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);
    println!("");

    //Loop labels to disambiguate between multiple loops
    //loop labels must begin with single quote (')
    let mut counter2 = 0;
    'counting_up: loop {
        println!("counter2: {}", counter2);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter2 == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter2 += 1;
    }

    println!("End Counter2: {}", counter2);
    println!("");

    //while loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
    println!("");

    //while used to loop over th elements of a collection, such as an array
    // it's more idiomatic to use a for loop, which we'll cover in the next section
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the a[{}] value is: {}", index, a[index]);
        index += 1;
    }

    println!("");

    //for loop for looping over a collection
    let a2 = [10,20,30,40,50];
    for element in a2 {
        println!("the value is: {}", element);
    }
    println!("");

    //for loop with range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    println!("");

    //excercise1
    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");
    
}