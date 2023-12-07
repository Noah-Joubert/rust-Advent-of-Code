use std::fs::read_to_string;
use std::char::from_u32;

fn main() {
    let file_data = read_to_string("src/bin/day3.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    // loop over every line
    let mut total = 0;
    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();

        // loop over every char
        let mut j = 0;
        while j < line.len() {
            // until we find a number
            if !line.chars().nth(j).unwrap().is_digit(10) {
                j += 1;
                continue;
            }

            // find the entire number
            let mut adjacent = false;
            let mut k = j;
            while (k < line.len()) && (line.chars().nth(k).unwrap().is_digit(10)) {
                if !adjacent {
                    // search adjacent cells
                    let x: i32 = i as i32;
                    let y: i32 = k as i32;
                    for (a, b) in vec![
                        (x - 1, y - 1), (x - 1, y), (x - 1, y + 1),
                        (x, y - 1), (x, y + 1),
                        (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]
                    {
                        if (a >= 0) && (a < lines.len() as i32) && (b >= 0) && (b < line.len() as i32) {
                            let adj_char = lines.get(a as usize).unwrap()
                                .chars().nth(b as usize).unwrap();
                            if (adj_char != '.') && (!adj_char.is_digit(10)) {
                                adjacent = true;
                            }
                        }
                    }
                }

                k += 1;
            }

            // add it to the total
            if adjacent {
                total += line.get(j..k).unwrap().parse::<i32>().unwrap();
            }

            j = k + 1;
        }
    }
    println!("Part 1: {total}");

    /* Part 2 */
    total = 0;
    let mut numbers: Vec<i32> = vec![];
    let mut hashed_lines: Vec<String> = vec![];
    let unicode_offset = 500;
    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();
        let mut new_line: String = "".to_string();

        // loop over every char
        let mut j = 0;
        while j < line.len() {
            // until we find a number
            let this_char = line.chars().nth(j).unwrap();
            if !this_char.is_digit(10) {
                j += 1;
                new_line += &this_char.to_string();
                continue;
            }

            // find the entire number
            let mut k = j;
            while k < line.len() {
                let next_char = line.chars().nth(k).unwrap();
                if next_char.is_digit(10) {
                    k += 1;
                    new_line += &from_u32((unicode_offset + numbers.len()) as u32).unwrap().to_string();
                } else {
                    break;
                }
            }

            numbers.push(line.get(j..k).unwrap().parse::<i32>().unwrap());
            j = k;
        }
        hashed_lines.push(new_line.clone());
    }
    for i in 0..lines.len() {
        let line = hashed_lines.get(i).unwrap();

        // loop over every char
        let mut j = 0;
        while j < line.chars().collect::<Vec<_>>().len() {
            // until we find a special character
            let this_char = line.chars().nth(j).unwrap();
            if this_char.is_digit(10) || this_char == '.' || this_char as u32 >= 400  {
                j += 1;
                continue;
            }

            // search adjacent cells
            let mut adj_nums = vec![];
            let x: i32 = i as i32;
            let y: i32 = j as i32;
            for (a, b) in vec![
                (x - 1, y - 1), (x - 1, y), (x - 1, y + 1),
                (x, y - 1), (x, y + 1),
                (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]
            {
                if (a >= 0) && (a < lines.len() as i32) && (b >= 0) && (b < line.len() as i32) {
                    let adj_char = hashed_lines.get(a as usize).unwrap()
                        .chars().nth(b as usize).unwrap();
                    if (adj_char as i32 - 400) >= 0 {
                        if !adj_nums.contains(&(adj_char as u32)) {
                            adj_nums.push(adj_char as u32);
                        }
                    }
                }
            }

            // add it to the total
            if adj_nums.len() == 2 {
                total += numbers[adj_nums[0] as usize - unicode_offset] * numbers[adj_nums[1] as usize - unicode_offset];
            }

            j = j + 1;
        }
    }
    println!("Part 2: {total}");
}