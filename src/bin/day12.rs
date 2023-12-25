use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashMap;

fn recur(mut springs: Vec<char>, mut groups: Vec<usize>, hash_map: &mut HashMap<String, u64>) -> u64 {
    let to_return =
        if springs.len() == 0 {
            // no springs left
            if groups.len() == 0 {
                1
            } else {
                0
            }
        } else if groups.len() == 0 {
            // no groups left
            if springs.contains(&'#') {
                0
            } else {
                1
            }
        } else {
            let key: String = groups.clone().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("") +
                &*springs.clone().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");

            let hash_probe = hash_map.get(&key);
            if hash_probe.is_some() {
                return *hash_probe.unwrap();
            }

            let next = springs.remove(0);
            let mut result = 0;
            if next == '.' || next == '?' {
                // just remove the first spring
                result += recur(springs.clone(), groups.clone(), hash_map);
            }
            if next == '#' || next == '?' {
                // traverse to the end of this group
                let size = groups.remove(0);
                let inc =
                    match springs.get(0..size - 1) {
                        None => 0,
                        Some(run) =>
                        if run.contains(&'.') {
                                0
                            } else {
                                match springs.get(size - 1) {
                                    Some(t) =>
                                        if *t == '#' {
                                            0
                                        } else {
                                            recur(springs.get(size..).unwrap().to_vec(), groups.clone(), hash_map)
                                        },
                                    None =>
                                        if groups.is_empty() {
                                            1
                                        } else {
                                            0
                                        }
                                }
                            }
                    };
                 result += inc;
            }
            hash_map.insert(key, result);
            result
        };
    return to_return;
}

fn main() {
    let file_data = read_to_string("src/bin/inputs/day12.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    let start = Instant::now();
    let mut total = 0;
    for line in &lines {
        let _space_split: Vec<&str> = line.split(" ").collect();
        let groups: Vec<usize> = _space_split[1].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let springs: Vec<char> = _space_split[0].chars().collect();

        let mut hash_map= HashMap::new();
        let sub_total = recur(springs, groups, &mut hash_map);
        total += sub_total;
    }
    let end = Instant::now();
    println!("Part 1: {total} in {:?}", end - start);

    /* Part 2 */
    let start = Instant::now();
    let mut total = 0;
    for line in &lines {
        let _space_split: Vec<&str> = line.split(" ").collect();
        let mut _og_groups: Vec<usize> = _space_split[1].split(",").map(|c| c.parse::<usize>().unwrap()).collect();
        let mut _og_springs: Vec<char> = _space_split[0].chars().collect();
        let mut groups: Vec<usize> = _og_groups.clone();
        let mut springs: Vec<char> = _og_springs.clone();
        for _ in 0..4 {
            groups.append(&mut _og_groups.clone());
            springs.push('?');
            springs.append(&mut _og_springs.clone());
        }

        let mut hash_map= HashMap::new();
        let sub_total = recur(springs, groups, &mut hash_map);
        total += sub_total;
    }
    let end = Instant::now();
    println!("Part 2: {total} in {:?}", end - start);
}