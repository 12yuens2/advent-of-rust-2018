use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
struct Claim {
    id: String,
    x: usize,
    y: usize,
    width: usize,
    height: usize
}

impl Claim {
    fn new(id: &str, x: &usize, y: &usize, width: &usize, height: &usize) -> Claim {
        Claim {
            id: id.to_string(),
            x: *x,
            y: *y,
            width: *width,
            height: *height
        }
    }
}

fn parse_claim(line: &str) -> Claim {
    let mut components = line.split(" ");
    let order_id = components.next().unwrap();
    components.next().unwrap(); // @

    let mut coordinates = components.next().unwrap().split(",");
    let x = coordinates.next().unwrap().parse::<usize>().unwrap();
    let mut y_str = coordinates.next().unwrap().to_string();
    let y_len = y_str.len();
    y_str.truncate(y_len - 1);
    let y = y_str.parse::<usize>().unwrap();

    let mut dimensions = components.next().unwrap().split("x");
    let width = dimensions.next().unwrap().parse::<usize>().unwrap();
    let height = dimensions.next().unwrap().parse::<usize>().unwrap();

    //println!("id: {}, x: {}, y: {}, width: {}, height: {}", order_id, x, y, width, height);

    Claim::new(&order_id, &x, &y, &width, &height)
}

fn use_fabric(claim: &Claim, fabric: &mut [[usize; 1000];1000]) {
    for i in claim.x..claim.x+claim.width {
        for j in claim.y..claim.y+claim.height {
            fabric[i][j] += 1;
        }
    }
}

fn check_overlap(claim: &Claim, fabric: &[[usize; 1000];1000]) {
    let mut overlap = false;
    for i in claim.x..claim.x+claim.width {
        for j in claim.y..claim.y+claim.height {
            if fabric[i][j] > 1 {
                overlap = true;
            }
        }
    }

    if !overlap {
        println!("{} claim id: {}", overlap, claim.id);
    }
}

fn main() {
    let mut f = File::open("input.data").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");

    let lines = contents.split("\n");

    let claims: Vec<Claim> = lines.map(|line| parse_claim(line)).collect();
    let mut fabric = [[0; 1000]; 1000];

    for claim in claims.clone() {
        use_fabric(&claim, &mut fabric);
    }

    for claim in claims {
        check_overlap(&claim, &fabric);
    }

    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            let num_claims = fabric[i][j];
            //print!("{}", num_claims);
            if num_claims > 1 {
                count += 1;
            }
        }
        //print!("\n");
    }
    println!("count: {}",count);
}
