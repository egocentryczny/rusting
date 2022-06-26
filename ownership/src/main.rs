fn main() {
    

    // //when 's' comes into scope it is valid
    // //and it remains valid until it goes out of scope
    // let s = "hello";

    // let mut ss = String::from("hello");
    // ss.push_str(", world!");
    // println!("{}", ss);


    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);


    let mut s = String::from("hello");

    some_fancy_function_here(s.clone());

    another_fancy_function(s.clone());

    s = some_return_function(s);
    println!("{}", s);
    println!("{}", s);

    // let reference_to_nothing = dangle();

    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }


fn some_fancy_function_here(some_string: String) {
    println!("{}", some_string);
}

fn another_fancy_function(some_string_too: String) {
    println!("{}", some_string_too);
}

fn some_return_function(some_string: String) -> String {
    some_string
}