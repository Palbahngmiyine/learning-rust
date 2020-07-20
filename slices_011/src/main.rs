fn main() {
    /*let mut s = String::from("hello world");
    // Gets 5
    let word = first_word(&s);
    s.clear(); */

    let s1 = String::from("hello world");

    let hello = &s1[0..5];
    let world = &s1[6..11];
    println!("hello: {}, world: {}", hello, world);

    // can skip first & last index
    /*let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];*/

    // can skip all indexes, two statements are same code
    // let slice = &s[0..len];
    // let slice = &s[..];

    /*let mut s = String::from("hello world");
    // Gets hello, word borrowed ownership
    let word = first_word(&s);
    s.clear(); // Error!
    println!("the first word is: {}", word);*/

    // It is slice and String literal
    //let sl = "Hello, world!";

    //let s2 = String::from("hello world");
    let s2 = "hello world";
    // Gets hello, word borrowed ownership
    let word = first_word(&s2);
    println!("hello: {}", word);

    // Other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("i32 first: {}, i32 second: {}", slice[0], slice[1]);
}

// first type of slice first word
/*fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}*/

// second type of slice first word
/*fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}