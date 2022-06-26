fn main() {
    let celc100: f64 = 100.0;
    let celc0: f64 = 0.0;
    println!("100c in fahr is {}", from_celc_to_fahr(celc100));
    println!("0c in fahr is {}", from_celc_to_fahr(celc0));

    let fahr32: f64 = 32.0;
    let fahr200: f64 = 200.0;
    println!("32 fahr in c is {}", from_fahr_to_celc(fahr32));
    println!("200 fahr in c is {}", from_fahr_to_celc(fahr200));
}

fn from_fahr_to_celc(fr: f64) -> f64 {
    (fr - 32.) * 5. / 9.
}

fn from_celc_to_fahr(ce: f64) -> f64 {
    ce * 9. / 5. + 32.
}

