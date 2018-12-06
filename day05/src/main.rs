use std::fs::File;
use std::io::prelude::*;
use std::mem;


fn check_case(first: &char, second: &char) -> bool {
    if first.is_uppercase() && second.is_lowercase() {
        return second.to_uppercase().collect::<Vec<_>>().get(0) == Some(first);
    }
    else if first.is_lowercase() && second.is_uppercase() {
        return first.to_uppercase().collect::<Vec<_>>().get(0) == Some(second);
    }
    return false;
}

fn check_letter(first: &char, second: &char, filter: &char) -> bool {
    if first.to_lowercase().collect::<Vec<_>>().get(0) == Some(filter) && second.to_lowercase().collect::<Vec<_>>().get(0) == Some(filter) {

        return check_case(first, second);
    }
    else {
        return false;
    }
}

fn react(cs: &mut Vec<char>, check: &Fn(&char, &char) -> bool) -> Vec<char> {
    let mut changed = true;
    while changed {
        let mut is: Vec<usize> = Vec::new();
        for i in 0..cs.len() {
            let j = i + 1;
            if j == cs.len() {
                break;
            }

            let first = cs.get(i).unwrap();
            let second = cs.get(j).unwrap();

            if check(first, second) {
                if !is.contains(&j) && !is.contains(&i) {
                    is.push(i);
                    is.push(j);
                }
            }
        }

        changed = is.len() != 0;

        for i in is.iter() {
            mem::replace(&mut cs[*i], '?');
        }
        cs.retain(|&x| x != '?');
    }

    return cs.to_vec();
}

fn main() {
    let mut f = File::open("input.data").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");

    //contents = "dabAcCaCBAcCcaDA".to_string();
    let cs = contents.chars().collect::<Vec<char>>();

    //part one
    let mut poly = cs.to_vec();
    react(&mut poly, &check_case);
    println!("Len: {}", poly.len());

    //part two
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    for c in alphabet.chars() {
        let mut polymer = cs.to_vec();
        polymer.retain(|letter| letter.to_lowercase().collect::<Vec<_>>().get(0) != Some(&c));
        react(&mut polymer, &check_case);
        println!("{}: {}", c, polymer.len());

        //react(&mut poly, &|first, second| check_letter(first, second, c));
        //println!("{:?}", poly);
        //println!("{:?}", poly);
    }
}
