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

fn main() {
    let mut f = File::open("input.data").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");

    let mut cs = contents.chars().collect::<Vec<char>>();
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

            if check_case(first, second) {
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

    println!("{:?}", cs);
    println!("Hello, world! {:?}", cs.len());
}
