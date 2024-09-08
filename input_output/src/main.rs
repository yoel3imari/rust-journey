use std::io;

fn main() {
    println!("Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Try again plz!");

    println!("Enter your age:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Try again plz!");
    let age: u32 = age.trim().parse().expect("Enter a number!");

    println!("Welcome: {}, you are {} years old", name, age);
}

fn shadowing() {
    let x: i32 = 5;
    let x = x + 1;
    // inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // x = 12
    }
    // x = 6
    println!("The value of x is: {x}");
}

/*

varibales are immutable by default
use mut to make it mutabel

references are immutable by default even if the variable is mutable
use &mut to to make the variable mutable inside the function
if the varibale is immutable you can't reference it as mutable

const is used to hold a pre defined value
can't use mut with const

shadowing is usefull when casting to diffrent
type because mutating type is not allowed even for mutable vars

*/
