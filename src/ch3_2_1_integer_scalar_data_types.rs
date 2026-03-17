pub fn run(){
    //scalar types: integers
    let a: i32 = 10; // 32-bit signed integer
    println!("Integer a: {}", a);

    let age:u8 = 36; // 8-bit unsigned integer
    println!("Age: {}", age);
    let count:i64 = 100000; // 64-bit signed integer
    println!("Count: {}", count);

    //Integer Literals
    let decimal = 98_222; // Decimal
    let hex = 0xff; // Hexadecimal
    let octal = 0o77; // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A'; // Byte (u8 only)
    println!("Decimal: {}, Hex: {}, Octal: {}, Binary: {}, Byte: {}", decimal, hex, octal, binary, byte);

    // let x: u8 = 255;
    // let y = x +1; // This will cause a compile-time error due to overflow
    // println!("y: {}", y);

    //Array cannot contain different types, but we can use a tuple to store different types of values
    // let message = "The temperature today is:";
    // let x = [message, 100];
    // println!("{} {}", x[0], x[1]);

    let t = ([1; 2], [3; 4]);
    let (a, _b) = t;
    println!("{}", a[0] + t.1[0]);
}