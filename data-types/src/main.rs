fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    println!("The value of five_hund is: {}", five_hundred);

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b = [3; 5];
    
}
