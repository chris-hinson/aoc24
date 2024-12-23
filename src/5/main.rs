pub fn main() {
    println!("{}", pt1_new());
}

pub fn pt1() -> usize {
    let input = std::fs::read_to_string("./src/5/input_dummy").expect("Unable to read file");
    let mut rules = input.lines().collect::<Vec<&str>>();
    let updates = rules
        .split_off(rules.iter().position(|e| *e == "").unwrap())
        .into_iter()
        .skip(1)
        .map(|s| {
            s.split(",")
                .map(|e| str::parse::<usize>(e).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let mut befores: Vec<Vec<usize>> = vec![Vec::new(); 100];
    let mut afters: Vec<Vec<usize>> = vec![Vec::new(); 100];

    //println!("{:?}\n{:?}", rules, pages);

    for rule in rules {
        let ele = rule
            .split("|")
            .map(|e| str::parse::<usize>(e).unwrap())
            .collect::<Vec<usize>>();
        let before = ele[0];
        let after = ele[1];
        befores[after].push(before);
        afters[before].push(after);
    }
    //println!("befores: {:?}", befores);
    let _b = befores
        .iter()
        .enumerate()
        .inspect(|(index, values)| println!("{}: {:?}\n", index, values))
        .collect::<Vec<_>>();
    //println!("afters: {:?}", afters);
    let _a = afters
        .iter()
        .enumerate()
        .inspect(|(index, values)| println!("{}: {:?}\n", index, values))
        .collect::<Vec<_>>();

    let mut middles: Vec<usize> = Vec::new();
    for u in updates.iter() {
        let bad = u.iter().enumerate().any(|(index, page)| {
            //dont examine this element if theres no rules that touch it
            if befores[*page].len() == 0 && afters[*page].len() == 0 {
                return false;
            }

            println!("{:?}", page);
            let preceeding_pages = &u[0..index];
            println!("{:?}", preceeding_pages);

            let following_pages = &u[index + 1..];
            println!("{:?}", following_pages);

            //if any of the preceeding elements are present in this afters list
            let a = preceeding_pages.iter().any(|v| afters[*page].contains(v));
            let b = following_pages.iter().any(|v| befores[*page].contains(v));
            a || b
        });

        /*if !bad {
            middles.push(update[updates.len() / 2]);
        }*/
    }

    println!("{:?}", middles);
    return middles.iter().sum();
}

pub fn pt1_new() -> usize {
    let input = std::fs::read_to_string("./src/5/input").expect("Unable to read file");
    let mut lines = input.lines().collect::<Vec<&str>>();
    let updates = lines
        .split_off(lines.iter().position(|e| *e == "").unwrap())
        .into_iter()
        .skip(1)
        .map(|s| {
            s.split(",")
                .map(|e| str::parse::<usize>(e).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    let rules = lines
        .into_iter()
        .filter(|e| *e != "")
        .map(|line| {
            println!("{:?}", line);
            line.split("|")
                .map(|e| str::parse::<usize>(e).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let total = updates
        .iter()
        .map(|update| {
            println!("{:?}", update);
            if rules
                .iter()
                .map(|rule| {
                    //println!("rule {},{}", rule[0], rule[1]);
                    let mut u_iter_a = update.iter();
                    let mut u_iter_b = u_iter_a.clone();
                    let a = u_iter_a.position(|e| *e == rule[0]);
                    let b = u_iter_b.position(|e| *e == rule[1]);

                    if a.is_none() || b.is_none() {
                        return false;
                    }

                    //println!("{} is at position {:?}", rule[0], a);
                    //println!("{} is at position {:?}", rule[1], b);

                    a > b || b < a
                })
                .any(|e| e)
            {
                //println!("rejecting bc {} is at {} and {} is at {}, which violates {} ");
                0
            } else {
                update[update.len() / 2]
            }
        })
        .collect::<Vec<_>>();

    //println!("\n\n{:?}", total);
    return total.iter().sum();
}
