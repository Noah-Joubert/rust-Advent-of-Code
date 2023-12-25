use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day13.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    let start = Instant::now();
    let mut part_1 = 0;
    let mut part_2 = 0;

    // collect the puzzles together
    let mut puzzles: Vec<Vec<Vec<char>>> = vec![];
    let mut i = 0;
    loop {
        let mut j = i;
        while j < lines.len() {
            if lines[j] == "" {
                break;
            }
            j += 1;
        }

        puzzles.push(lines.get(i..j).unwrap().iter().map(|l| l.chars().collect()).collect());
        i = j + 1;
        if i >= lines.len() {
            break;
        }
    }

    for puzzle in puzzles {
        // build each row and column into a string to simplify comparisons
        let (mut cols, mut rows): (Vec<String>, Vec<String>) = (vec![], vec![]);
        let (n, m): (usize, usize) = (puzzle.len(), puzzle[0].len());
        for i in 0..n {
            let mut s: String = "".to_string();
            for j in 0..m {
                s += &puzzle[i][j].to_string();
            }
            rows.push(s);
        }
        for j in 0..m {
            let mut s: String = "".to_string();
            for i in 0..n {
                s += &puzzle[i][j].to_string();
            }
            cols.push(s);
        }

        // now check for reflections
        for (upper, arr, inc) in [(n, rows, 100), (m, cols, 1)] {
            for i in 0..(2 * upper - 3) {
                let middle = (i + 1) as f32 / 2.0;
                let mut step = if i % 2 == 0 {
                    0.5
                } else {
                    0.0
                };
                let mut differences = 0;
                while middle + step < upper as f32 && middle - step >= 0.0 {
                    for (a, b) in arr[(middle + step) as usize].chars().zip(arr[(middle - step) as usize].chars()) {
                        if a != b {
                            differences += 1;
                        }
                    }
                    if differences > 1 {
                        break;
                    }
                    step += 1.0;
                }
                if differences == 1 && i % 2 == 0 {
                    part_2 += inc * middle.ceil() as u64;
                }
                if differences == 0 {
                    part_1 += inc * middle.ceil() as u64;
                }
            }
        }
    }

    let end = Instant::now();

    println!("Part 1: {part_1} in {:?}", end - start);
    println!("Part 2: {part_2} in {:?}", end - start);
}