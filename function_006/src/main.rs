fn main() {
    println!("Hello, world!");

    another_function();
    another_parameter_function(5);

    // Rust must return value
    // let x = (let y = 6); 

    let x = 5;

    let y = {
        // Expression        
        let x = 3;
        x + 1    
    };
    println!("The Value of x is: {}, The value of y is: {}", x, y);

    println!("Five function returns: {}", five());
    println!("Plus one fucntion returns: {}", plus_one(3));
}

fn another_function() {
    println!("Another function.");
}

fn another_parameter_function(i: i32) {
    println!("Another function with integer: {}", i);
}

fn five() -> i32 {
    5
}

// Like arrow function in Javascript
// Don't add ; in last statements, that returns empty tuple
fn plus_one(x: i32) -> i32 {
    x + 1
}