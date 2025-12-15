// None of the concepts presented in this chapter are unique to Rust

fn main() {
    // Constant example (my name thats not change)
    const FABRICIO: &str = "Fabricio Lennart Flores Ledezma";
    let mut x = 'a';
    println!("The value of x is: {x}");
    x = 'b';
    println!("The value of x is: {x}");
    println!("This is a constant: {FABRICIO}");
    // shadowing example
    let spaces = "__";
    println!("{spaces}");
    println!("The spaces variable is a diferent type now but we use the same variable name");
    let spaces = spaces.len();
    println!("{spaces}")
    // data types
}
