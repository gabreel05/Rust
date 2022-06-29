fn main() {
    println!("Hello World!");

    another_function();

    let x = sum(15, 15);
    let y = plus_one(11);

    println!("The return value is {}", x);
    println!("The value plus one is {}", y);
}

fn another_function() {
    println!("Another print!");
}


fn sum(x: i32, y: i32) -> i32 {
    return x + y
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
