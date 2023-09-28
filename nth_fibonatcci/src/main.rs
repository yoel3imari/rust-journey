use std::io;

fn main() {
    loop {
        println!("Choose nth number: ");
        let mut nth = String::new();
        io::stdin().read_line(&mut nth).expect("Invalid input!");
        let nth: u32 = match nth.trim().parse::<u32>() {
            Ok(nth) => nth,
            Err(_) => continue,
        };
        let result = nth_fibonacci(nth);
        if nth <= 1 {
            println!("The first Fibonacci is {result}");
        } else if nth == 2 {
            println!("The second Fibonacci is {result}");
        } else {
            println!("The {nth}nth Fibonacci is {result}");
        }
    }
}

fn nth_fibonacci (nth: u32) -> u32 {
    if nth <= 1 {
        return 0;
    } else if nth == 2 {
        return 1
    } else {
        let mut a = 0;
        let mut b = 1;

        for _ in 3..=nth {
            let c = a + b;
            a = b;
            b = c;
        }

        return b;
    }
}
