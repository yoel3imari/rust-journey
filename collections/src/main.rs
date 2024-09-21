fn main() {
    // creation
    let _v: Vec<i32> = Vec::new();
    let mut vv = vec![1, 2, 3];

    // add elm
    vv.push(4);

    let _third = &vv[2];
    let fourth = vv.get(5);

    match fourth {
        Some(data) => println!("fourth : {data}"),
        None => println!("no fourth"),
    };

    // only take a value from a vec when you're about to use
    // let t = &vv[0];
    // vv.push(6);
    // # t is out of range here
    // println!("t => {t}");
    
    
    // iteration
    for i in &vv {
        print!("{i}");
    }
    println!();

    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = "World!";
    let s3 = "يوسف العيماري";
    let s4 = s1 + s2 + s3;

    let i = 2;
    let m = 8;

    /* not to use */
    let slc = &s3[0..2];

    /* instead use for loop to get single chars or bytes */
    for (c, i) in s4.chars().enumerate() {
        println!("{i} => {c}");
    }

}
