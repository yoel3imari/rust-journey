fn main() {
    // tuple
    // let t: (i32, f64, char) = (23, 23.0, 'C');

    // destructuring
    // let (x, y, z) = t;

    // println!("x = {}", x);

    // array
    // let t = [1, 2, 3];
    // let c: [char; 2] = ['A', 'c'];
    // println!("c0 = {}", c[0]);

    // loops()
    // forloop();

    // functions
    // fn name(params: type, ...) -> returnType {}

    // borrowing

    // this code will not work because s1 ownership has been moved to takes_ownership 
    // and ended with the end of the function call
    let s1 = String::from("Hello world");
    /*let lenght = takes_ownership(s1);
    println!("s1: {s1} , len: {lenght}")*/

    // to fix this we pass s1 as a reference to the function
    let len = takes_reference(&s1);
    println!("s1: {s1} , len: {len}");
    // this code works perfectly
    // if takes_references needs to mutate s1 it will not because it's not mutable and 
    // the reference is not mutable too
    // to mutate a reference, the variables passed needs to be mut and the ref also should be mut
    // a variable can have only one reference at a time

    // we can multiple immutable references but only one mutable reference at a time
    // tho we cannot have both mutable and immut ref a the same time
    let concat_len = multi_immut_refs(&s1, &s1);
    println!("result: {concat_len}");

    /* SLICES */
    let s = String::from("HELLO WORLD");
    let word = first_word(&s);
    println!("word = {word}");

}

fn other(x: i32, y: i32) -> i32 {
    x + y
}

fn loops() {
    let mut counter = 0;
    loop {
        counter += 1;

        if counter > 1_000_000 {
            break;
        }
        println!("{}", counter);
    }
}

fn forloop() {
    let arr = [1, 2, 3, 4];
    for a in arr {
        println!("{}", a);
    }
}

fn takes_ownership(s: String) -> usize {
    s.len()
}

fn takes_reference(s: &String) -> usize {
    s.len()
}

fn multi_immut_refs(s1: &String, s2: &String) -> usize {
    s1.len() + s2.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..]
}