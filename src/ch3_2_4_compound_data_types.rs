pub fn run() {
    // Coumpound data types: tuples and arrays

    // Tuples
    let tup = (500, 6.4, 'z');
    let (x, y, z) = tup; // destructuring
    println!("The value of x from destructured tuple: {}", x);
    println!("The value of y from destructured tuple: {}", y);
    println!("The value of z from destructured tuple: {}", z);
    println!("---");

    //Indexing
    println!("The value of first element from tuple: {}", tup.0);
    println!("The value of second element from tuple: {}", tup.1);
    println!("The value of third element from tuple: {}", tup.2);
    println!("---");
    
    // Arrays
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    println!("The first element of the array is: {}", first);

    // let arr = [1,2,3];
    // println!("{}", arr[10]);// This will cause a runtime panic due to index out of bounds

    let zeros = [0; 5]; // Create an array of 5 zeros
    println!("The zeros array: {:?}", zeros);
}