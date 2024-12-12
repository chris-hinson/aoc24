#![feature(iter_map_windows)]
pub fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

pub fn part1() -> usize {
    std::fs::read_to_string("./src/2/input")
        .expect("Unable to read file")
        .lines()
        .map(|e| e.split_whitespace().collect::<Vec<&str>>())
        .map(|e| {
            e.into_iter()
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|e| e.iter().is_sorted() || e.iter().rev().is_sorted())
        .map(|line| {
            line.iter()
                .map_windows(|[a, b]| (b.clone() - a.clone()).abs())
                .collect::<Vec<i64>>()
        })
        .map(|line| {
            line.iter().find(|&&e| e > 3).is_none() && line.iter().find(|&&e| e == 0).is_none()
        })
        .filter(|e| *e == true)
        .count()
}

pub fn part2() -> usize {
    let input = std::fs::read_to_string("./src/2/input_dummy").expect("Unable to read file");

    let nums = input
        .lines()
        .map(|e| e.split_whitespace().collect::<Vec<&str>>())
        .map(|e| {
            e.into_iter()
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .inspect(|line| println!("{:?}", line))
        .collect::<Vec<Vec<i64>>>();

    for (i, row) in nums.iter().enumerate() {
        println!("inspecting row {i}");
        let mut remove_used = false;
        for w in row.windows(3) {
            //a < b < c - good
            //a < b > c - bad
            //a > b < c - good
            //a > b > c - bad

            let l_diff = w[1] - w[0];
            let r_diff = w[2] - w[1];

            let good =
                ((w[0] < w[1] && w[1] < w[2]) || (w[0] > w[1] && w[1] > w[2])) && l_diff <= 3;

            if good {
                continue;
            }

            if !good && !remove_used {
                remove_used = true;
            } else {
                break;
            }
        }
    }
    0
}

/*    let input = std::fs::read_to_string("./src/2/input").expect("Unable to read file");

let res = input
    .lines()
    .map(|e| e.split_whitespace().collect::<Vec<&str>>())
    .map(|e| {
        e.into_iter()
            .map(|f| f.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    })
    //    .collect::<Vec<Vec<i64>>>();
    //let inc_dec = nums
    //    .into_iter()
    .filter(|e| e.iter().is_sorted() || e.iter().rev().is_sorted())
    //.inspect(|row| println!("{:?}", row))
    //    .collect::<Vec<Vec<i64>>>();
    //let diffs = inc_dec
    //    .into_iter()
    .map(|line| {
        line.iter()
            .map_windows(|[a, b]| (b.clone() - a.clone()).abs())
            .collect::<Vec<i64>>()
    })
    //.inspect(|line| println!("diffs: {:?}", line))
    .map(|line| {
        line.iter().find(|&&e| e > 3).is_none() && line.iter().find(|&&e| e == 0).is_none()
    })
    //.inspect(|line| println!("mapped to bool: {}", line))
    .filter(|e| *e == true)
    .count();
println!("part1: {res}");
*/
