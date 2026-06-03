pub fn run(){
    move_ownership();
    println!("");

    borrowing();
    println!("");

    borrowing_mutable();
    println!("");

    mut_immut_reference();
    println!("");

    let s = get_string();
    println!("{}", s);
}

//exercise move ownership
fn move_ownership() {
    let s1 = String::from("move_ownership");
    // let s2 = s1; // s1 is moved to s2, and s1 is no longer valid
    let _s2 = s1.clone();

    println!("{}", s1); //if s1 is moved to s2, this line will cause a compile error because s1 is no longer valid
}

// exercise borrowing with references
fn borrowing() {
    let s = String::from("borrowing");

    borrowing_print_string(&s);

    println!("{}", s); // This works because s is borrowed, not moved, borrowing is use & symbol.
}

fn borrowing_print_string(s: &String) {
    println!("{}", s);
}

//exercise borrowing with mutable references
fn borrowing_mutable() {
    let mut s = String::from("hello");

    append_text(&mut s);

    println!("{}", s);
}

fn append_text(s: &mut String) {
    s.push_str(" world");
}

//exercise mutable and immutable reference cannt be in the same scope at the same time
fn mut_immut_reference() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s;

    println!("{}", r3);
}

//exercise return reference
fn get_string() -> String {
    let s = String::from("hello");
    s // ✅ correct
}