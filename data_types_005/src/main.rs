fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // Type Annotation needed
    // let guess = "42".parse().expect("Not a number!");
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("Integer: {}, F64 Float number: {}, F32 Float number: {}", guess, x, y);

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    println!("Addition: {}, Subtraction: {}, Multiplication: {}, Division: {}, Remainder: {}", sum, difference, product, quotient, remainder);

    // without type annotation
    let t = true;
    // with explicit type annotation
    let f: bool = false;
    println!("True: {}, False: {}", t, f);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Plain text: {}, Special Char: {}, Emoji: {}", c, z, heart_eyed_cat);

    // Tuple is just complex type, will bind separate variables
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple type X: {}, Tuple type y: {}, Tuple type Z: {}", x, y, z);

    // Direct access to Tuple
    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;
    println!("Tuple 1: {}, Tuple2: {}, Tuple3: {}", five_hundred, six_point_four, one);

    // Array must be equal type each data
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("Today is: {}", months[6]);

    // Wrong way of array access 
    // let a = [1, 2, 3, 4, 5];
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);
}
