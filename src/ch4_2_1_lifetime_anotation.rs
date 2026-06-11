pub fn run() {
    let s1 = String::from("abcd");

    let s2 = String::from("xy");
    let result = longest(&s1, &s2);

    println!("{}", result);
}

// lifetime annotation is a way to specify how long references are valid in Rust. 
// It helps the compiler ensure that references do not outlive the data they point to, 
// preventing dangling references and ensuring memory safety.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}