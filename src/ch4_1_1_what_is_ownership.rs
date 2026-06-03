pub fn run() {
    
    //since rust has 2 type
    //copy type : i32, bool, char, f64, etc. these types are stored on the stack and are copied when assigned to another variable.
    //non-copy type : String, Vec, etc. these types are stored on the heap and are moved when assigned to another variable, meaning the original variable is no longer valid.
    let a = true;
    read(a);

    owner_i32();
    
    //clone
    let first = String::from("Finsa");
    let first_clone = first.clone();
    let full_name = add_suffix(first_clone);
    println!("{}, originally {}", full_name, first);

    //ownership
    ownership_1();
    ownership_2();
}

fn read(x: bool) {
    if x {
        println!("x is true");
    }
}

fn owner_i32() {
    let x = 5;
    let y = x; // i32 is Copy, so x is still valid after this line
    println!("x: {}", x);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn ownership_1() 
{
    let s1 = String::from("hello");
    
    {
        //let s2 = s1; // s1 is moved to s2, and s1 is no longer valid
        let s2 = s1.clone(); // s1 is cloned to s2, and s1 is still valid
    }
    println!("{}", s1);
}

fn ownership_2() {
    let s1 = String::from("hello");// s1 is created and owns the string "hello" on heap

    let s2 = s1.clone();// s1 is cloned to s2, so both s1 and s2 own their own copy of the string "hello" on heap

    println!("{s1}");
    println!("{s2}");
}