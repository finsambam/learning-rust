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

    //exercise: 
    //Note: All elements of a tuple and an array must have the same type, but the types of the elements in a tuple can be different, 
    //while the types of the elements in an array must be the same. Create a tuple and an array with different types of elements, and print their values.
    //character 'A' has a Unicode code point of 65, so we can convert it to a u8 by using the `as` keyword. Then we can convert it to f64 for the array.
    let exe_tup = (10, 2.5, 'A');
    let (exe_x, exe_y, exe_z) = exe_tup;
    let exe_arr = [exe_x as f64, exe_y, exe_z as u8 as f64];
    println!("exe_arr: {:?}", exe_arr);

    let exe_c = 'Z';
    let exe_n = exe_c as u8;

    println!("exe_n: {}", exe_n);


}