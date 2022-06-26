fn main() {
    
    let mut song = String::new();

    let total = 12;
    let mut current = 1;
    while current <= total {
        song.push_str(header_generator(current).as_str());
        song.push('\n');
        song.push_str(line_generator(current).as_str());
        song.push('\n');
        song.push('\n');
        current +=1;
    }
    println!("Song: {}", song);

}

fn header_generator(part_number: u64) -> String {
    let headers: [&str; 12] = [
        "1st",
        "2nd",
        "3rd",
        "4th",
        "5th",
        "6th",
        "7th",
        "8th",
        "9th",
        "10th",
        "11th",
        "12th"
    ];
    format!("On the {} day of Christmas, my true love sent to me", headers[part_number as usize -1])
}

fn line_generator(part_number: u64) -> String {
    let lines = [
        "And a partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    if part_number == 0 {
        return String::new();
    }
    let mut ret = String::new();
    let mut starting_line = part_number;
    while starting_line >= 1 {
        ret.push_str(lines[starting_line as usize -1]);
        ret.push('\n');
        starting_line -= 1;
    }
    return ret;
}