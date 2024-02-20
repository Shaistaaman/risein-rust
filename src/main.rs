fn main() {
    println!("Hello, world!");


// Variable types
let message = "Hello, world!";

let x: i32 = 42;
let pi: f64 = 3.14159;
let is_rust_fun: bool = true;
let letter_a: char = 'a';


// Function
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// rust expression
let x = 42;

if x >= 0 {
	println!("x is non-negative");
} else {
      println!("x is negative");
}

// while loop

let mut i = 1;

while i <= 5 {
	println!("{}", i);
      i += 1;
}
}