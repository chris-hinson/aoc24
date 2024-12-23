use regex::Regex;
pub fn main() {
    //pt1();
    pt2();
}

fn pt1() -> i64 {
    let input = std::fs::read_to_string("./src/3/input").expect("Unable to read file");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res = re
        .captures_iter(&input)
        .inspect(|e| println!("{:?}", e))
        .inspect(|e| println!("{:?} {:?}", e.get(1), e.get(2)))
        .map(|e| {
            e.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * e.get(2).unwrap().as_str().parse::<i64>().unwrap()
        })
        .fold(0, |a, b| a + b);

    println!("res: {res}");
    res
}

fn pt2() -> i64 {
    let input = std::fs::read_to_string("./src/3/input").expect("Unable to read file");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res = input
        .split("do()")
        .map(|s| s.split("don't()"))
        .inspect(|e| println!("{:?}", e))
        .map(|mut e| {
            re.captures_iter(&e.next().unwrap())
                //.inspect(|e| println!("{:?}", e))
                //.inspect(|e| println!("{:?} {:?}", e.get(1), e.get(2)))
                .map(|e| {
                    e.get(1).unwrap().as_str().parse::<i64>().unwrap()
                        * e.get(2).unwrap().as_str().parse::<i64>().unwrap()
                })
                .fold(0, |a, b| a + b)
        })
        .sum();

    println!("res: {res}");
    res
}
