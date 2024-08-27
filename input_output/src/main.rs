use std::io;

fn main() {
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line( &mut name).expect("Try again plz!");
    
    println!("Enter your age:");
    let mut age = String::new();
    io::stdin().read_line( &mut age).expect("Try again plz!");
    let age: u32 = age.trim().parse().expect("Enter a number!");

    println!("Welcome: {}, you are {} years old", name, age);
}
