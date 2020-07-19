fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);


    let mut s = String::from("hello");
    println!("{}", s);
    change(&mut s);
    println!("{}", s);

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world")
}

fn dangle() -> String {
    let s = String::from("hello, world!");

    s
}