fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    println!("X: {}", x);

    let x = x + 1;
    println!("After new let: {}", x);
    {
        let x = x * 2;
        println!("In curly:{}", x);
    }
    println!("after curly: {}", x);

    println!("The const: {}", THREE_HOURS_IN_SECONDS)
}
