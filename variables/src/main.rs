const PI: f32 = 3.14;
static mut HELLO: &str = "Hello";

fn main() {
    let mut a = 10;
    println!("{a}");
    a = 100;
    println!("{a}");

    let a = "string"; // shadowing

    // PI = 3.15;

    unsafe {
        HELLO = "Hello 1";
    }

    let snake_case = "Tên biến";
}
