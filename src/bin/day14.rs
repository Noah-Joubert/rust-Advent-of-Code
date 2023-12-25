use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day14.txt").unwrap();
    let mut puzzle: Vec<Vec<char>> = file_data.split("\n").map(|s| s.chars().collect()).collect();

    /* Part 1 */
    let start = Instant::now();
    let mut total = 0;
    let (n, m) = (puzzle.len(), puzzle[0].len());
    for j in 0..m {
        let mut moved = true;
        while moved {
            moved = false;
            for i in 1..n {
                if puzzle[i][j] == 'O' && puzzle[i - 1][j] == '.' {
                    puzzle[i][j] = '.';
                    puzzle[i - 1][j] = 'O';
                    moved = true;
                }
            }
        }

        for i in 0..n {
            if puzzle[i][j] == 'O' {
                total += n - i;
            }
        }
    }
    let end = Instant::now();
    println!("Part 1: {total} in {:?}", end - start);


    /* Part 2 */
    let start = Instant::now();
    let mut puzzle: Vec<Vec<char>> = file_data.split("\n").map(|s| s.chars().collect()).collect();
    let mut total = 0;
    let (n, m) = (puzzle.len(), puzzle[0].len());
    let (mut range_1, mut range_2, mut range_3, mut range_4): (Vec<Vec<(usize, usize)>>, Vec<Vec<(usize, usize)>>, Vec<Vec<(usize, usize)>>, Vec<Vec<(usize, usize)>>) = (vec![], vec![], vec![], vec![]);
    for j in 0..m {
        let mut v = vec![];
        for i in 1..n {
            v.push((i, j));
        }
        range_1.push(v.clone());

        let mut v = vec![];
        for i in (0..n-1).rev() {
            v.push((i, j));
        }
        range_3.push(v.clone());
    }
    for i in 0..n {
        let mut v = vec![];
        for j in 1..m {
            v.push((i, j));
        }
        range_2.push(v.clone());

        let mut v = vec![];
        for j in (0..m-1).rev() {
            v.push((i, j));
        }
        range_4.push(v.clone());
    }
    let mut period: i32 = -1;
    let mut hash: HashMap<String, usize> = HashMap::new();
    for cycle in 0..1000000000 {
        if period == -1 {
            let key: String = puzzle.iter().map(|v| v.iter().collect::<String>()).collect();
            let hash_probe = hash.get(&*key);
            if hash_probe.is_some() {
                period = (cycle - hash_probe.unwrap()) as i32;
            } else {
                hash.insert(key, cycle);
            }
        } else if (1000000000 % period) == (cycle as i32 % period) {
            break;
        }
        for (range, step) in [(range_1.clone(), (-1, 0)), (range_2.clone(), (0, -1)), (range_3.clone(), (1, 0)), (range_4.clone(), (0, 1))] {
            for direction in range {
                let mut moved = true;
                while moved {
                    moved = false;
                    for (i_ref, j_ref) in &direction {
                        let (i, j) = (*i_ref, *j_ref);
                        let (step_i, step_j) = ((i as i32 + step.0) as usize, (j as i32 + step.1) as usize);
                        if puzzle[i][j] == 'O' && puzzle[step_i][step_j] == '.' {
                            puzzle[i][j] = '.';
                            puzzle[step_i][step_j] = 'O';
                            moved = true;
                        }
                    }
                }
            }
        }
    }
    for j in 0..m {
        for i in 0..n {
            if puzzle[i][j] == 'O' {
                total += n - i;
            }
        }
    }
    let end = Instant::now();
    println!("Part 2: {total} in {:?}", end - start);
}