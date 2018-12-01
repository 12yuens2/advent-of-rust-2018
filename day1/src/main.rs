use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;


fn main() {

    let mut frequency = 0;
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(frequency);

    let mut f = File::open("input.data").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error reading file");

    while true {
        let lines = contents.split("\n");
        for line in lines {
            frequency = parse_line(frequency, line.to_string());

            if seen_frequencies.contains(&frequency) {
                println!("Frequency already in set: {}", frequency);
                exit(0);
            }
            else {
                seen_frequencies.insert(frequency);
            }
        }
    }
}


fn parse_line(frequency: i32, line: String) -> i32 {
    let mut chars = line.chars();
    let sign = chars.next().unwrap();
    let number = chars.collect::<String>();

    if sign == '+' {
        return frequency + number.parse::<i32>().unwrap();
    }
    else {
        return frequency - number.parse::<i32>().unwrap();
    }

}
