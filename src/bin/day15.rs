use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day15.txt").unwrap();
    let puzzle: Vec<&str> = file_data.split(['\n', ',']).collect();

    /* Part 1 */
    let start = Instant::now();
    let mut total: i64 = 0;
    for s in &puzzle {
        let mut sub_val = 0;
        for c in s.chars() {
            sub_val = ((sub_val + c as i64) * 17) % 256;
        }
        total += sub_val;
    }
    println!("Part 1: {total} in {:?}", Instant::now() - start);

    /* Part 2 */
    let start = Instant::now();
    total = 0;
    let mut boxes: HashMap<u32, Vec<(String, u32)>> = HashMap::new();
    for s in puzzle {
        let operation = if s.contains('-') {
            '-'
        } else if s.contains('=') {
            '='
        } else {
            panic!("No operation.")
        };

        let _split: Vec<&str> = s.split(operation).collect();
        let key_str: String = _split[0].to_string();
        let mut key: u32 = 0;
        for c in key_str.chars() {
            key = ((key + c as u32) * 17) % 256;
        }
        let mut queue: Vec<(String, u32)> = match boxes.get(&key) {
            Some(t) => t.clone(),
            None => vec![]
        };

        if operation == '-' {
            for (index, e) in queue.clone().iter().enumerate() {
                if e.0 == key_str {
                    queue.remove(index);
                }
            }
        } else if operation == '=' {
            let val: u32 = _split[1].parse::<u32>().unwrap();
            let mut found = false;
            for (index, e) in queue.clone().iter().enumerate() {
                if e.0 == key_str {
                    found = true;
                    queue.remove(index);
                    queue.insert(index, (key_str.to_string(), val))
                }
            }
            if !found {
                queue.push((key_str.to_string(), val));
            }
        }
        boxes.insert(key, queue.clone());
    }
    for (i, b) in boxes {
        for (j, (_, val)) in b.iter().enumerate() {
            total += ((i + 1) * (j as u32 + 1) * (*val)) as i64;
        }
    }

    let end = Instant::now();
    println!("Part 2: {total} in {:?}", end - start);
}