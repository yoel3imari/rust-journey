fn main() {
    let s = String::from("hello world");

    let first_word = first_word(&s);

    let slice = &s[2..5];

    // s = String::from("new value");

    println!("first word is {first_word} slice is {slice}");
}

fn first_word(s: &String) -> &str {
    let arr = s.as_bytes();

    for (i, &item) in arr.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

