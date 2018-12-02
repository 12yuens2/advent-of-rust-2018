use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;

extern crate distance;
use distance::*;

fn part1(contents: &str) {
    println!("Part 1:");

    let mut count_two = 0;
    let mut count_three = 0;

    let lines = contents.split("\n");
    for line in lines {
        let chars = line.chars();
        let mut char_dict = HashMap::new();

        for c in chars {
            let mut count = 0;
            if char_dict.contains_key(&c) {
                count = *char_dict.get(&c).unwrap();
            }
            char_dict.insert(c, count+1);
        }

        let twos = char_dict.values().filter(|x| **x == 2).count();
        if twos > 0 {
            count_two += 1;
        }

        let threes = char_dict.values().filter(|x| **x == 3).count();
        if threes > 0 {
            count_three += 1;
        }

    }

    println!("twos: {}, threes: {}", count_two, count_three);
    println!("total: {}", count_two * count_three);
}

fn part2(contents: &str) {
    println!("Part two:");

    let lines = contents.split("\n");
    for pair1 in lines {
        for pair2 in contents.split("\n").filter(|x| *x != pair1) {
            let distance = levenshtein(pair1, pair2);
            if distance == 1 {
                // Stop when solution is found
                println!("pair: {},{}", pair1, pair2);
                return;
            }
        }
    }
}

fn main() {
    let mut f = File::open("input.data").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error reading file");

    part1(&contents);
    println!("--------------");
    part2(&contents);
}
