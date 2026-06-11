pub fn run() {
    //& symbol is reference or borrowing in rust, its make onwership of value not transfered to another variable.
    let s1 = String::from("hello");
    print_value(&s1); // pass a reference
    println!("{}", s1); // s1 is still valid after passing a reference

    //since we only want to count variable s1 length, we just need to borrow or referece s1, not move the ownership
    let length = str_length(&s1); // pass a reference to calculate length
    println!("Length of '{}': {}", s1, length); // s1 is still valid after passing a reference

    multiple_borrowing(); // demonstrate multiple borrowing

    scoped_mutable_borrowing(); // demonstrate mutable borrowing with scopes

    imutable_mutable_borrowing(); // demonstrate immutable and mutable borrowing

    // dangling_reference(); // demonstrate dangling reference

    not_dangling_reference(); // demonstrate how to avoid dangling reference
}

fn print_value(s: &String) {
    println!("{}", s);
}

fn str_length(s: &String) -> usize {
    s.len()
}

fn multiple_borrowing() {
    let x = String::from("test multiple borrowing");
    let x1 = &x; // first reference to x
    let x2 = &x; // second reference to x
    println!("value of x: {}, x1: {}, x2: {}", x, x1, x2); // all references are valid
}

// this function will cause compile error because we cannot have multiple mutable references to the same variable at the same time
//principe of borrowing: may readers or one writer, but not both at the same time
// fn wrong_mutable_borrowing() {
//     let mut s = String::from("hello");
//     let r1 = &mut s; // mutable reference
//     r1.push_str("world");
//     let r2 = &mut s; // another mutable reference
//     r2.push_str("!!!");
//     println!("r1: {}, r2: {}", r1, r2); // both references are valid
// }

// to fix the compile error, we can use scopes to ensure that only one mutable reference exists at a time
fn scoped_mutable_borrowing() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str("world");
    }
    {
        let r2 = &mut s;
        r2.push_str("!!!");
    }
    println!("s: {}", s);
}

//this example below is explain why borrowing rules are important, 
//because if we allow both immutable and mutable references at the same time, 
//it can cause race condition and dangling reference and data inconsistency, 
//because we cannot guarantee the order of access to the variable and the value of the variable may change unexpectedly
// fn wrong_imutable_mutable_borrowing() {
//     let mut a = String::from("test");

//     let b = &a;
//     let c = &mut a;// error: cannot borrow `a` as mutable because it is also borrowed as immutable

//     println!("b: {}", b);
//     println!("c: {}", c);
// }

fn imutable_mutable_borrowing() {
    let mut a = String::from("test");

    let b = &a;
    println!("b: {}", b);
    
    let c = &mut a;
    c.push_str("borrowing!!!");
    println!("c: {}", c);
}

// fn dangling_reference() {
//     let r;

//     {
//         let s = String::from("hello");
//         r = &s; // r is a reference to s, but s will go out
//     }
//     println!("r: {}", r); // error: `s` does not live long enough, r is a dangling reference
// }

fn not_dangling_reference() {
    let s = String::from("hello");

    {
        let r = &s;
        println!("{}", r);
    }

    println!("{}", s);
}