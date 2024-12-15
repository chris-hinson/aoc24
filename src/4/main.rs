fn main() {
    //println!("{}", pt1());
    println!("{}", pt2());
}

fn pt1() -> usize {
    let input = std::fs::read_to_string("./src/4/input").expect("Unable to read file");

    //transform into vecvecchar
    let board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let HEIGHT: usize = board.len();
    let WIDTH: usize = board[0].len();

    let mut finds: Vec<(usize, usize)> = Vec::new();

    for (row_i, row) in board.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col != 'X' {
                continue;
            }

            //n
            if row_i >= 3 {
                let f = vec![
                    board[row_i][col_i],
                    board[row_i - 1][col_i],
                    board[row_i - 2][col_i],
                    board[row_i - 3][col_i],
                ]
                .into_iter()
                .collect::<String>();
                if f == "XMAS" {
                    finds.push((row_i, col_i));
                }
            }
            //ne
            if row_i >= 3 && col_i < WIDTH - 3 {
                let f = vec![
                    board[row_i][col_i],
                    board[row_i - 1][col_i + 1],
                    board[row_i - 2][col_i + 2],
                    board[row_i - 3][col_i + 3],
                ]
                .into_iter()
                .collect::<String>();
                if f == "XMAS" {
                    finds.push((row_i, col_i));
                }
            }
            //e
            if col_i < WIDTH - 3 {
                let f = vec![
                    board[row_i][col_i],
                    board[row_i][col_i + 1],
                    board[row_i][col_i + 2],
                    board[row_i][col_i + 3],
                ]
                .into_iter()
                .collect::<String>();
                if f == "XMAS" {
                    finds.push((row_i, col_i));
                }
            }
            //se
            if col_i < WIDTH - 3 && row_i < HEIGHT - 3 {
                let f = vec![
                    board[row_i][col_i],
                    board[row_i + 1][col_i + 1],
                    board[row_i + 2][col_i + 2],
                    board[row_i + 3][col_i + 3],
                ]
                .into_iter()
                .collect::<String>();
                if f == "XMAS" {
                    finds.push((row_i, col_i));
                }
            }
            //s
            if row_i < HEIGHT - 3 {
                let f = vec![
                    board[row_i][col_i],
                    board[row_i + 1][col_i],
                    board[row_i + 2][col_i],
                    board[row_i + 3][col_i],
                ]
                .into_iter()
                .collect::<String>();
                if f == "XMAS" {
                    finds.push((row_i, col_i));
                }
            }
            //sw
            if row_i < HEIGHT - 3 && col_i >= 3 {
                let f = vec![
                    board[row_i][col_i],
                    board[row_i + 1][col_i - 1],
                    board[row_i + 2][col_i - 2],
                    board[row_i + 3][col_i - 3],
                ]
                .into_iter()
                .collect::<String>();
                if f == "XMAS" {
                    finds.push((row_i, col_i));
                }
            }
            //w
            if col_i >= 3 {
                let f = vec![
                    board[row_i][col_i],
                    board[row_i][col_i - 1],
                    board[row_i][col_i - 2],
                    board[row_i][col_i - 3],
                ]
                .into_iter()
                .collect::<String>();
                if f == "XMAS" {
                    finds.push((row_i, col_i));
                }
            }
            //nw
            if row_i >= 3 && col_i >= 3 {
                let f = vec![
                    board[row_i][col_i],
                    board[row_i - 1][col_i - 1],
                    board[row_i - 2][col_i - 2],
                    board[row_i - 3][col_i - 3],
                ]
                .into_iter()
                .collect::<String>();
                if f == "XMAS" {
                    finds.push((row_i, col_i));
                }
            }
        }
    }
    return finds.len();
}

fn pt2() -> usize {
    let input = std::fs::read_to_string("./src/4/input").expect("Unable to read file");

    //transform into vecvecchar
    let board = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let HEIGHT: usize = board.len();
    let WIDTH: usize = board[0].len();

    let mut finds: Vec<(usize, usize)> = Vec::new();

    for (row_i, row) in board.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if *col != 'A' || (col_i < 1 || col_i > WIDTH - 2) || (row_i < 1 || row_i > HEIGHT - 2)
            {
                continue;
            }

            //nw
            let nw_char = board[row_i - 1][col_i - 1];

            //ne
            let ne_char = board[row_i - 1][col_i + 1];

            //se
            let se_char = board[row_i + 1][col_i + 1];

            //sw
            let sw_char = board[row_i + 1][col_i - 1];

            if (nw_char == 'M' && ne_char == 'M') && (se_char == 'S' && sw_char == 'S') {
                finds.push((row_i, col_i));
            }
            if (ne_char == 'M' && se_char == 'M') && (nw_char == 'S' && sw_char == 'S') {
                finds.push((row_i, col_i));
            }
            if (se_char == 'M' && sw_char == 'M') && (ne_char == 'S' && nw_char == 'S') {
                finds.push((row_i, col_i));
            }
            if (sw_char == 'M' && nw_char == 'M') && (ne_char == 'S' && se_char == 'S') {
                finds.push((row_i, col_i));
            }
        }
    }

    /*for (i, row) in board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if finds.iter().find(|e| **e == (i, j)).is_some() {
                print!("{col}");
            } else {
                print!(".")
            }
        }
        println!("");
    }*/

    return finds.len();
}
