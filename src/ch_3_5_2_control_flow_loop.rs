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
}