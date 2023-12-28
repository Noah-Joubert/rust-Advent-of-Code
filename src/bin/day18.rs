use std::fs::read_to_string;
use std::time::Instant;
use std::cmp::{max, min};
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day18.txt").unwrap();
    let puzzle: Vec<&str> = file_data.split("\n").collect();

    let start = Instant::now();
    let (mut i, mut j) = (0, 0);

    /* Part 1 */
    // read in the steps
    let mut steps: Vec<(char, i32)> = vec![];
    for l in &puzzle {
        let _split: Vec<&str> = l.split(' ').collect();
        let (dir, step) = (_split[0], _split[1].parse::<i32>().unwrap());
        steps.push((dir.parse::<char>().unwrap(), step));
    }

    // find the extent of the map
    let mut coords: Vec<(i128, i128)> = vec![(0, 0)];
    let (mut max_i, mut min_i, mut max_j, mut min_j) = (0, 0, 0, 0);
    for (dir, mut step) in &steps {
        while step > 0 {
            (i, j) = match dir {
                'R' => (i, j + 1),
                'L' => (i, j - 1),
                'D' => (i + 1, j),
                'U' => (i - 1, j),
                _ => panic!("Unexpected direction")
            };
            coords.push((i, j));

            min_i = min(i, min_i);
            max_i = max(i, max_i);
            max_j = max(j, max_j);
            min_j = min(j, min_j);
            step -= 1;
        }
    }

    // fill in the map
    let (n, m) = (1 + (max_i - min_i) as usize, 1 + (max_j - min_j) as usize);
    let mut map: Vec<Vec<char>> = vec![vec![' '; m]; n];
    for (i, j) in coords {
        map[(i - min_i) as usize][(j - min_j) as usize] = 'O';
    }

    // find the enclosed squares (using day 10 code)
    let mut to_visit: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        to_visit.push((i, 0));
        to_visit.push((i, m - 1));
    }
    for j in 0..m {
        to_visit.push((0, 0));
        to_visit.push((n - 1, j));
    }
    while !to_visit.is_empty() {
        let (i, j) = to_visit.pop().unwrap();
        if map[i][j] != ' ' { continue; }

        map[i][j] = 'X';
        if i >= 1 { to_visit.push((i - 1, j)); }
        if i + 1 < n { to_visit.push((i + 1, j)); }
        if j >= 1 { to_visit.push((i, j - 1)); }
        if j + 1 < m { to_visit.push((i, j + 1)); }
    }
    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] != 'X' {
                total += 1;
            }
        }
    }
    let end = Instant::now();
    println!("Part 1: {total} in {:?}", end - start);

    /* Part 2 */
    let start = Instant::now();
    // read in and make the steps, store each line segment's start and end points
    let mut pos: (i64, i64) = (0, 0);
    let mut line_segments: Vec<((i64, i64), (i64, i64))> = vec![];
    for l in &puzzle {
        let _split: Vec<&str> = l.split(' ').collect();

        let dir_hex = _split[2].chars().nth(7).unwrap().to_string().parse::<u8>().unwrap();
        let step_hex_str = _split[2].get(2..7).unwrap();
        let (dir, step): (char, i64) = (match dir_hex { 0 => 'R', 1 => 'D', 2 => 'L', 3 => 'U', _ => panic!("Unexpected hex character")},
                           i64::from_str_radix(step_hex_str, 16).unwrap());

        let end_pos = match dir {
            'R' => (pos.0, pos.1 + step),
            'L' => (pos.0, pos.1 - step),
            'U' => (pos.0 - step, pos.1),
            'D' => (pos.0 + step, pos.1),
            _ => panic!("Unexpected direction")
        };

        let mut end_points = vec![pos, end_pos];
        end_points.sort();
        let (p1, p2) = (end_points[0], end_points[1]);
        line_segments.push((p1, p2));

        pos = end_pos;
    }

    // find and sort the coordinates of the vertices
    let mut vertices: Vec<(i64, i64)> = line_segments.iter().fold(vec![], |mut a, b| {a.append(&mut vec![b.0, b.1]); a});
    vertices.sort_by(|a, b| a.cmp(b));
    let x_vertices: Vec<i64> = {
        let mut _temp: Vec<i64> = vertices.clone().into_iter().map(|p| p.0).unique().collect();
        _temp.sort();
        _temp
    };
    let y_vertices: Vec<i64> = {
        let mut _temp: Vec<i64> = vertices.clone().into_iter().map(|p| p.1).unique().collect();
        _temp.sort();
        _temp
    };

    // now build the scaled grid which has each row and column scaled to a certain size
    // we have one row and column for every vertex, and one between each vertex
    // the rows and columns for each vertex are not scaled, but the rows and columns between are
    let mut scaled_grid: Vec<Vec<char>> = vec![vec![' '; 2*y_vertices.len() - 1]; 2*x_vertices.len() - 1];
    let x_scaling: Vec<i64> = {
        let mut v = vec![];
        for i in 0..(x_vertices.len() - 1) {
            v.push(1);
            v.push(x_vertices[i + 1] - x_vertices[i] - 1);
        }
        v.push(1);
        v
    };
    let y_scaling: Vec<i64> = {
        let mut v = vec![];
        for i in 0..(y_vertices.len() - 1) {
            v.push(1);
            v.push(y_vertices[i + 1] - y_vertices[i] - 1);
        }
        v.push(1);
        v
    };
    let x_mapping: HashMap<i64, usize> = {
        let mut _temp = HashMap::new();
        for i in 0..x_vertices.len() {
            _temp.insert(x_vertices[i], 2*i);
        }
        _temp
    };
    let y_mapping: HashMap<i64, usize> = {
        let mut _temp = HashMap::new();
        for i in 0..y_vertices.len() {
            _temp.insert(y_vertices[i], 2*i);
        }
        _temp
    };

    // fill the grid with the line segments
    for (start, end) in &line_segments {
        let scaled_start = (*x_mapping.get(&start.0).unwrap(), *y_mapping.get(&start.1).unwrap());
        let scaled_end = (*x_mapping.get(&end.0).unwrap(), *y_mapping.get(&end.1).unwrap());

        // we don't need to worry about if order (eg. scaled_start.0 > scaled_start.1) in these ranges as the line segment positions are sorted
        for i in (scaled_start.0)..(scaled_end.0 + 1) {
            for j in (scaled_start.1)..(scaled_end.1 + 1) {
                scaled_grid[i][j] = 'X';
            }
        }
    }

    // find the enclosed squares (using day 10 code)
    let (n, m) = (scaled_grid.len(), scaled_grid[0].len());
    let mut to_visit: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        to_visit.push((i, 0));
        to_visit.push((i, m - 1));
    }
    for j in 0..m {
        to_visit.push((0, 0));
        to_visit.push((n - 1, j));
    }
    while !to_visit.is_empty() {
        let (i, j) = to_visit.pop().unwrap();
        if scaled_grid[i][j] != ' ' { continue; }

        scaled_grid[i][j] = 'O';
        if i >= 1 { to_visit.push((i - 1, j)); }
        if i + 1 < n { to_visit.push((i + 1, j)); }
        if j >= 1 { to_visit.push((i, j - 1)); }
        if j + 1 < m { to_visit.push((i, j + 1)); }
    }

    // sum up the empty space
    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if scaled_grid[i][j] != 'O' {
                total += x_scaling[i] * y_scaling[j];
            }
        }
    }

    let end = Instant::now();
    println!("Part 2: {total} in {:?}", end - start);
}