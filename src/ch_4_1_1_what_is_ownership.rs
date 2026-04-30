pub fn run() {
    let first = String::from("Finsa");
    let full_name = add_suffix(first);
    println!("{}", full_name);
}

pub fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}