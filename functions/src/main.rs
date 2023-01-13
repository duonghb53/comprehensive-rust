fn main() {
    print_hello();
    let number = add_one(1);
    println!("{number}");
}

pub fn add_one(number: i32) -> i32 {
    number + 1
}

fn print_hello() {
    println!("Hello, world!");
}
