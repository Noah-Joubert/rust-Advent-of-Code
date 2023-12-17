use std::fs::read_to_string;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day9.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    let mut part1_total = 0;
    let mut part2_total = 0;
    for line in lines {
        let starting_sequence: Vec<i64>  = line.split(' ').map(|c| c.parse::<i64>().unwrap()).collect();
        let mut sequence_differences: Vec<Vec<i64>> = vec![starting_sequence.clone()];

        // calculate the sequence_differences until we get all zeroes
        let mut current_sequence = starting_sequence;
        let mut all_zeroes: bool = false;
        while !all_zeroes {
            let mut new_sequence: Vec<i64> = vec![];
            all_zeroes = true;
            for i in 0..current_sequence.len() - 1 {
                new_sequence.push(current_sequence[i + 1] - current_sequence[i]);
                if *new_sequence.last().unwrap() != 0 { all_zeroes = false; }
            }
            sequence_differences.push(new_sequence.clone());
            current_sequence = new_sequence;
        }

        let mut diff: i64 = 0;
        for i in (0..sequence_differences.len() - 1).rev() {
            diff += *sequence_differences[i].last().unwrap();
        }
        part1_total += diff;

        let mut diff: i64 = 0;
        for i in (0..sequence_differences.len() - 1).rev() {
            diff = sequence_differences[i][0] - diff;
        }
        part2_total += diff;
    }
    println!("Part 1: {part1_total}");
    println!("Part 2: {part2_total}");
}