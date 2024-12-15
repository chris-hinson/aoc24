pub fn main() {
    println!("{}", pt1());
}

pub fn pt1() -> usize {
    let input = std::fs::read_to_string("./src/5/input").expect("Unable to read file");
    let mut rules = input.lines().collect::<Vec<&str>>();
    let pages = rules
        .split_off(rules.iter().position(|e| *e == "").unwrap())
        .into_iter()
        .map(|s| {
            s.split(",")
                .map(|e| str::parse::<usize>(e).unwrap())
                .collect::<Vec<usize>>()
        });

    let constraints: Vec<Vec<usize>> = vec![Vec::new(); 100];

    println!("{:?}\n{:?}", rules, pages);

    0
}
