fn main() {
    // println!("Hello, world!");
    // let a:u8 = 10; // --> 255, 0 | 1

    // let b = 30_000_000;

    // let f = 5.3;

    // let isType = true;

    // let c = 'a';
    // let emoji = '😀';

    // let mut str = "Hello World!";
    // let mut strObj = String::from("xin chào");
    // strObj.push_str(str);

    let arr: [u8; 3] = [1, 2, 3];
    // let item1 = arr[0];
    let item3 = arr[2];

    let tuple = (1, true, "hello");
    let item1 = tuple.0;
    let (number, isTrue, str) = tuple;
    println!("{str}");
}
