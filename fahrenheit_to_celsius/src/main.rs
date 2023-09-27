use std::io;

fn main() {
    // use enter fahren
    // validate input
    // convert to celsius (separated fn)
    // print result
    println!("Welcome!");

    loop {
        // enter temper
        let mut fhrnht = String::new();
        println!("enter temperature in fahrenheit:");
        io::stdin()
            .read_line(&mut fhrnht)
            .expect("not valide! try again");

        // convert to f64
        // specify to parse the type you want to get
        let fhrnht: f64 = match fhrnht.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // convert to celsius
        let clss = convert_to_celsius(fhrnht);
        println!("{fhrnht} in celsius is : {}", format!("{:.2}", clss));
        // break;
    }
}

fn convert_to_celsius(t: f64) -> f64 {
    return (t - 32.0) * (5.0 / 9.0);
}
