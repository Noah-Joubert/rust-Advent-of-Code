use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day16.txt").unwrap();
    let mut puzzle: Vec<Vec<char>> = file_data.split(['\n', ',']).map(|s| s.chars().collect()).collect();

    /* Part 1 */
    let start = Instant::now();
    let (n, m) = (puzzle.len(), puzzle[0].len());

    let mut starting_beams: Vec<Vec<((usize, usize), (i32, i32))>> = vec![];
    for i in 0..n {
        starting_beams.push(vec![((i, 0), (0, 1))]);
        starting_beams.push(vec![((i, m-1), (0, -1))]);
    }
    for j in 0..m {
        starting_beams.push(vec![((0, j), (1, 0))]);
        starting_beams.push(vec![((n-1, j), (-1, 0))]);
    }

    let mut totals: Vec<i32> = vec![];
    for mut beams in starting_beams {
        let mut memory: HashMap<String, bool> = HashMap::new();
        let mut first: bool = true;
        let mut energy_map: Vec<Vec<bool>> = vec![vec![false; m]; n];

        while !beams.is_empty() {
            let (mut pos, mut step): ((usize, usize), (i32, i32)) = beams.pop().unwrap();

            loop {
                // check if we are out of bounds, and make the step
                let (mut i, mut j): (i32, i32) =
                    if !first {
                        (pos.0 as i32 + step.0, pos.1 as i32 + step.1)
                    } else {
                        first = false;
                        (pos.0 as i32, pos.1 as i32)
                    };
                if !((0 <= i) && (i < n as i32) && (0 <= j) && (j < m as i32)) {
                    break;
                }
                pos = (i as usize, j as usize);
                energy_map[pos.0][pos.1] = true;

                // check if we've done this before
                let key = pos.0.to_string() + " " + &pos.1.to_string() + " " + &step.0.to_string() + " " + &step.1.to_string();
                if memory.get(&key).is_some() {
                    break
                }
                memory.insert(key, true);

                // check for a reflector/ splitter
                if (puzzle[pos.0][pos.1] == '\\') || (puzzle[pos.0][pos.1] == '/') {
                    step = if step.0 == 1 {
                        (0, 1)
                    } else if step.0 == -1 {
                        (0, -1)
                    } else if step.1 == 1 {
                        (1, 0)
                    } else if step.1 == -1 {
                        (-1, 0)
                    } else {
                        panic!("Unknown step!");
                    };
                    if puzzle[pos.0][pos.1] == '/' {
                        step = (-step.0, -step.1);
                    }
                } else if (puzzle[pos.0][pos.1] == '-') && (step.0 != 0) {
                    beams.push((pos, (0, 1)));
                    beams.push((pos, (0, -1)));
                    break;
                } else if (puzzle[pos.0][pos.1] == '|') && (step.1 != 0) {
                    beams.push((pos, (1, 0)));
                    beams.push((pos, (-1, 0)));
                    break;
                }
            }
        }
        let total = energy_map.iter().map(|v| v.iter().map(|b| if *b {1} else {0}).sum::<i32>()).sum();
        totals.push(total);
    }
    let end = Instant::now();
    println!("Part 1: {} in {:?}", totals[0], end - start);
    println!("Part 2: {:?} in (the same time)", totals.iter().max().unwrap());
}