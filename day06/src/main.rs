use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Coordinate {
    id: String,
    x: usize,
    y: usize
}

impl Coordinate {
    fn new(x: &usize, y: &usize, id: &usize) -> Coordinate {
        Coordinate {
            id: id.to_string(),
            x: *x,
            y: *y
        }
    }
}

fn parse_coordinate(line: &str, count: &mut usize) -> Coordinate {
    *count += 1;
    let mut coord = line.split(",");
    let x = coord.next().unwrap().parse::<usize>().unwrap();
    let y = coord.next().unwrap().trim().parse::<usize>().unwrap();

    return Coordinate::new(&x, &y, count);
}

fn get_max_xy(coords: Vec<Coordinate>) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;

    for coord in coords {
        let x = coord.x;
        let y = coord.y;

        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    return (max_x + 1, max_y + 1);
}

fn calc_distance(x: &usize, y: &usize, coordinate: &Coordinate) -> usize {
    let x_distance = *x as i32 - coordinate.x as i32;
    let y_distance = *y as i32 - coordinate.y as i32;

    return (x_distance.abs() + y_distance.abs()) as usize;
}

fn calc_closest_coord(x: &usize, y: &usize, coords: Vec<Coordinate>) -> String {
    let mut distances: Vec<(String,usize)> = coords.iter()
        .map(|ref coord| (coord.id.to_string(),calc_distance(x, y, &coord)))
        .collect();

    distances.sort_by(|a, b| a.1.cmp(&b.1));

    if distances.get(0).unwrap().1 == distances.get(1).unwrap().1 {
        return "0".to_string();
    }
    else {
        return distances.get(0).unwrap().0.to_string();
    }
}

fn calc_sum_distance(x: &usize, y: &usize, coords: Vec<Coordinate>) -> usize {
    let distance = coords.iter()
        .map(|coord| calc_distance(x, y, &coord))
        .sum();
        //.collect();

    return distance;
}

fn main() {
    let mut f = File::open("input.data").expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Error reading file");

    let mut count = 0;
    let coords: Vec<Coordinate> = contents.split("\n")
        .map(|line| parse_coordinate(line, &mut count))
        .collect();

    let max = get_max_xy(coords.clone());
    let mut grid_raw = vec![0; max.0 * max.1];
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(max.1).collect();
    let mut grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    //part two
    let mut num_safe = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let distance = calc_sum_distance(&i, &j, coords.clone());
            if distance < 10000 {
                num_safe += 1
            }
        }
    }

    println!("{}", num_safe);

    //part one
    let mut filter: HashSet<usize> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let id = calc_closest_coord(&i, &j, coords.clone()).parse::<usize>().unwrap();
            grid[i][j] = id;

            if i == 0 || i == grid.len() - 1 || j == 0 || j == grid.len() - 1 {
                filter.insert(id);
            }
        }
    }

    let mut counts: HashMap<usize, usize> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let id = grid[i][j];
            if !filter.contains(&id) {
                let mut count = 0;
                if counts.contains_key(&id) {
                    count = *counts.get(&id).unwrap();
                }
                counts.insert(id, count+1);
            }
        }
    }

    println!("counts: {:?}", counts);


}
