use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, player!");
    
    let sec_num = rand::thread_rng().gen_range(1..=100);
    // println!("my num is: {sec_num}");

    loop {
        
        println!("Guess my number: ");
        
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Sorry, i couldn't hear ya");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };
    
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Go higher!"),
            Ordering::Greater => println!("Get Lower!"),
            Ordering::Equal => {
                println!("Boooyaaa you got that right! it's {guess}");
                break;
            }
        }
    
    }
    
}
