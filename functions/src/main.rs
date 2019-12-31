fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Can not have a ';' as we want the value of x + 1 to be returned. a semicolon will prevent that
}