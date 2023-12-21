use std::fs::read_to_string;
use num::abs;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day11.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();
    let universe: Vec<Vec<char>> = lines.iter().map(|s| (*s).chars().collect()).collect();

    let n = universe.len();
    let m = universe[0].len();

    // find the duplicated columns and rows
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut i_mapping_1: Vec<i64> = vec![];
    let mut j_mapping_1: Vec<i64> = vec![];
    let mut i_mapping_2: Vec<i64> = vec![];
    let mut j_mapping_2: Vec<i64> = vec![];
    let (mut i_shift_1, mut j_shift_1): (i64, i64) = (0, 0);
    let (mut i_shift_2, mut j_shift_2): (i64, i64) = (0, 0);
    for i in 0..n {
        let mut galaxy_found = false;
        for j in 0..m {
            if universe[i][j] == '#' {
                galaxies.push((i, j));
                galaxy_found = true;
            }
        }
        i_mapping_1.push(i as i64 + i_shift_1);
        i_mapping_2.push(i as i64 + i_shift_2);
        if !galaxy_found {
            i_shift_1 += 1;
            i_shift_2 += 999999;
        }
    }
    for j in 0..m {
        let mut galaxy_found = false;
        for i in 0..n {
            if universe[i][j] == '#' {
                galaxy_found = true;
            }
        }
        j_mapping_1.push(j as i64 + j_shift_1);
        j_mapping_2.push(j as i64 + j_shift_2);
        if !galaxy_found {
            j_shift_1 += 1;
            j_shift_2 += 999999;
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1.. galaxies.len() {
            let (i1, i2)= galaxies[i];
            let (j1, j2) = galaxies[j];
            let distance_1 = abs(i_mapping_1[i1] - i_mapping_1[j1]) + abs(j_mapping_1[j2] - j_mapping_1[i2]);
            let distance_2 = abs(i_mapping_2[i1] - i_mapping_2[j1]) + abs(j_mapping_2[j2] - j_mapping_2[i2]);
            part_1 += distance_1;
            part_2 += distance_2;
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}