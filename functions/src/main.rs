fn main() {
    println!("Hello, world!");
    some_function();
    some_getting_int(5);

    let y = {
        let x = 3;
        x + 1
    };

    println!("Y: {}", y);


    let mut count = 0;
    'counting_up: loop {
        println!("Count = {}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn some_function() {
    println!("Another function.");
}

fn some_getting_int(x: i32) {
    println!("The value of x is : {}", x);
}
