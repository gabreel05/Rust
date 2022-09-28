fn main() {
    println!("Hello, world!");
    println!("{}", sum(1, 2));
    println!("{}", subtract(2, 1));
}

// function to get the sum of two numbers
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}
