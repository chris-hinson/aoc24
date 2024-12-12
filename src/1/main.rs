use std::fs;

pub fn main() {
    let input = fs::read_to_string("./src/1/input").expect("Unable to read file");

    let lines = input.lines().collect::<Vec<&str>>();
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in lines {
        let lr: Vec<&str> = line.split("   ").collect::<Vec<&str>>();
        left.push(lr[0].parse::<i64>().unwrap());
        right.push(lr[1].parse::<i64>().unwrap());
    }
    left.sort();
    right.sort();

    let z = std::iter::zip(left.clone(), right.clone());
    let result1 = z.map(|e| (e.1 - e.0).abs()).fold(0, |acc, x| acc + x);
    println!("result: {result1}");

    let result2 = left
        .iter()
        .map(|e| e * right.iter().filter(|f| *f == e).count() as i64)
        .fold(0, |acc, e| acc + e);
    println!("result2: {result2}");
}
