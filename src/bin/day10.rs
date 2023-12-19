use std::fs::read_to_string;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day10.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|s| (*s).chars().collect()).collect();

    /* Part 1 */
    let n = lines.len();
    let m = lines[0].len();

    let mut part_2_grid: Vec<Vec<char>> = vec![vec![' '; 2*m + 1]; 2*n + 1];

    let mut initial_pos = (0, 0);
    for i in 0..n {
        let mut found = false;
        for j in 0..m {
            if grid[i][j] == 'S' {
                initial_pos = (i, j);
                found = true;
                break;
            }
        }
        if found { break; }
    };

    let mut pipe: Vec<char> = vec!['S'];
    let mut prev_pos = initial_pos;
    let mut pos = (initial_pos.0 + 1, initial_pos.1);
    loop {
        // (part 2 stuff)
        part_2_grid[2* pos.0 + 1][2* pos.1 + 1] = 'P';
        part_2_grid[pos.0 + prev_pos.0 + 1][pos.1 + prev_pos.1 + 1] = 'P';

        let current_char = grid[pos.0][pos.1];
        let temp = pos;
        if current_char == 'S' { break; }
        pos = {
            if current_char == '|' {
                if prev_pos.0 < pos.0 { (pos.0 + 1, pos.1) }
                else { (pos.0 - 1, pos.1) }
            } else if current_char == '-' {
                if prev_pos.1 < pos.1 { (pos.0, pos.1 + 1) }
                else { (pos.0, pos.1 - 1) }
            } else if current_char == 'L' {
                if prev_pos.0 < pos.0 { (pos.0, pos.1 + 1) }
                else { (pos.0 - 1, pos.1) }
            } else if current_char == 'J' {
                if prev_pos.0 < pos.0 { (pos.0, pos.1 - 1) }
                else { (pos.0 - 1, pos.1) }
            } else if current_char == '7' {
                if prev_pos.0 > pos.0 { (pos.0, pos.1 - 1) }
                else { (pos.0 + 1, pos.1) }
            } else if current_char == 'F' {
                if prev_pos.0 > pos.0 { (pos.0, pos.1 + 1) }
                else { (pos.0 + 1, pos.1) }
            } else {
                panic!("Unexpected!");
            }
        };

        prev_pos = temp;
        pipe.push(current_char);
    }
    println!("Part 1: {}", pipe.len() / 2);

    /* Part 2 */
    // start on the edges and path find inwards!
    let mut to_visit: Vec<(usize, usize)> = vec![];
    for i in 0..(2*n + 1) {
        to_visit.push((i, 0));
        to_visit.push((i, 2*m));
    }
    for j in 0..(2*m + 1) {
        to_visit.push((0, 0));
        to_visit.push((2*n, j));
    }
    while !to_visit.is_empty() {
        let (i, j) = to_visit.pop().unwrap();
        if part_2_grid[i][j] != ' ' { continue; }

        part_2_grid[i][j] = 'X';
        if i >= 1 { to_visit.push((i-1, j)); }
        if i < 2*n { to_visit.push((i+1, j)); }
        if j >= 1 { to_visit.push((i, j - 1)); }
        if j < 2*m { to_visit.push((i, j + 1)); }
    }
    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if part_2_grid[2*i + 1][2*j + 1] == ' ' {
                total += 1;
            }
        }
    }
    println!("Part 2: {total}");
}