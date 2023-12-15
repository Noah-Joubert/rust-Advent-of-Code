use std::fs::read_to_string;
use std::cmp::{min, max};

fn main() {
    let file_data = read_to_string("src/bin/day5.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    // get the first line of seeds
    let _first_line: Vec<&str> = lines[0]
        .split(" ")
        .collect();
    let seeds: Vec<i64> = _first_line
        .get(1..)
        .unwrap()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    // parse and process the maps
    let mut maps: Vec<Vec<(i64, i64, i64)>> = vec![];
    let map_lines = lines.get(3..).unwrap(); // start at file line of first map
    let mut front_of_map: usize = 0;
    while front_of_map < map_lines.len() {
        // find the lines which make up this map
        let mut end_of_map: usize = front_of_map;
        loop {
            match map_lines.get(end_of_map) {
                None => break,
                Some(s) =>
                    match *s {
                        "" => break,
                        _ => end_of_map += 1
                    }
            }
        }
        end_of_map = min(end_of_map,  map_lines.len() - 1); // clamp to end of file

        // save the map
        let mut this_map: Vec<(i64, i64, i64)> = vec![];
        for line in map_lines.get(front_of_map..end_of_map ).unwrap() {
            let range: Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            this_map.push((range[0], range[1], range[2]));
        }
        this_map.sort_by_key(|x| x.0); // sort for part 2
        maps.push(this_map);

        front_of_map = end_of_map + 2;
    }

    // run through the maps for each individual seed
    let mut result = -1;
    for mut seed in seeds {
        for map in &maps {
            for range in map {
                let (destination, source, range) = range;
                if (seed >= *source) && (source + range >= seed) {
                    seed = destination + (seed - source);
                    break;
                }
            }
        }
        if result == -1 || seed < result {
            result = seed;
        }
    }
    println!("Part 1: {result}");


    /* Part 2 */
    // get the first line of seed ranges
    let seeds: Vec<i64> = _first_line.get(1..).unwrap().iter().map(|x| x.parse::<i64>().unwrap()).collect();
    let mut seed_ranges: Vec<(i64, i64)> = vec![];
    for pair in seeds.chunks(2){
        seed_ranges.push((pair[0], pair[1]));
    }

    // run through the maps for each individual seed
    let mut repeat_seeds: Vec<(i64, i64)>;
    let mut next_seeds: Vec<(i64, i64)>;

    // loop through every group of mappings

    for map in &maps {
        next_seeds = vec![];

        // loop through every seed range
        while !seed_ranges.is_empty() {
            repeat_seeds = vec![];
            for (seed_start, seed_range) in &seed_ranges {
                // for every map in the group of mappings
                let mut intersection_found: bool = false;
                for (map_destination, map_start, map_range) in map {

                    // check for an intersection
                    let intersection: (&i64, i64) = (max(map_start, seed_start), min(seed_start + seed_range, map_start + map_range));
                    if intersection.0 > &intersection.1 { continue; } else { intersection_found = true; }

                    // add the left split, add the right split, map the middle split
                    if intersection.0 > seed_start {
                        repeat_seeds.push((*seed_start, intersection.0 - 1 - seed_start));
                    }
                    if intersection.1 < seed_start + seed_range {
                        repeat_seeds.push((intersection.1 + 1, seed_start + seed_range - (intersection.1 + 1)));
                    }
                    next_seeds.push((map_destination + (*intersection.0 - map_start), intersection.1 - intersection.0));
                }

                // if no intersections were found, this range isn't mapped
                if !intersection_found { next_seeds.push((*seed_start, *seed_range)); }
            }
            seed_ranges = repeat_seeds;
        }

        seed_ranges = next_seeds;
    }
    let result = seed_ranges.iter().map(|x| x.0).min().unwrap();
    println!("Part 2: {result}");
}
