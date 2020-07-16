fn main() {
    let s = "hello!";
    {
        // Isolated
        let s = "hello, World!";
        println!("{}", s);
    }
    println!("{}", s);
    // Must be mutable
    let mut s = String::from("hello");
    // Mutating string from origin string
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    // Copied
    let y = x;
    println!("{}", x);
    println!("{}", y);

    // Works different from i32 shadowing
    let s1 = String::from("hello");
    // Moved value, Rust provides moving ownership feature for Resolve of Memory leak bugs(ex) Double free error)
    let s2 = s1;
    // Prints borrow of moved value: `s1`
    // println!("{}", s1);

    // Rust doesn't provides deep copy feature automatically
    println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    // It works!
    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = String::from("hello");
    takes_ownership(s3);

    // Borrow of moved value s3
    // println!("{}", s3);

    let x = 5;
    makes_copy(x);

    let s4 = gives_ownership();
    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s5);
    println!("s4: {}", s4);
    // Moved s5 to s6
    // println!("s5: {}, s6: {}", s5, s6);
    println!("s6: {}", s6);

    let s7 = String::from("hello");
    let (s8, len) = calculate_length(s7);
    println!("The length of '{}' is {}.", s8, len);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}