use std::fs::read_to_string;
use std::cmp::max;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day2.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    let mut total = 0;
    let (fixed_red_max, fixed_green_max, fixed_blue_max) = (12, 13, 14);
    for line in &lines {
        let mut max_red: u32 = 0;
        let mut max_blue: u32 = 0;
        let mut max_green: u32 = 0;

        let split_on_colon: Vec<&str> = line.split(':').collect();
        let game_meta_data: Vec<&str> = split_on_colon[0].split(' ').collect();
        let game_index = (*game_meta_data[1]).parse::<u32>().unwrap();

        let quantity_colour_pairs: Vec<&str> = split_on_colon[1].split([',',';']).collect();
        for pair in quantity_colour_pairs {
            let split_pair_string: Vec<&str>= pair.trim().split(' ').collect();
            let quantity = split_pair_string[0].parse::<u32>().unwrap();
            let colour = split_pair_string[1];
            match colour {
                "red" => max_red = max(max_red, quantity),
                "blue" => max_blue = max(max_blue, quantity),
                _ => max_green = max(max_green, quantity)
            }
        }
        total += ((fixed_blue_max >= max_blue) && (fixed_green_max >= max_green) && (fixed_red_max >= max_red)) as u32 * game_index;
    }
    println!("Part 1: {total}");

    /* Part 2 */
    total = 0;
    for line in &lines {
        let mut max_red: u32 = 0;
        let mut max_blue: u32 = 0;
        let mut max_green: u32 = 0;

        let split_on_colon: Vec<&str> = line.split(':').collect();

        let quantity_colour_pairs: Vec<&str> = split_on_colon[1].split([',',';']).collect();
        for pair in quantity_colour_pairs {
            let split_pair_string: Vec<&str>= pair.trim().split(' ').collect();
            let quantity = split_pair_string[0].parse::<u32>().unwrap();
            let colour = split_pair_string[1];
            match colour {
                "red" => max_red = max(max_red, quantity),
                "blue" => max_blue = max(max_blue, quantity),
                _ => max_green = max(max_green, quantity)
            }
        }
        total += max_red * max_blue * max_green;
    }
    println!("Part 1: {total}");
}
