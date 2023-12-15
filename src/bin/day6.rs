use std::fs::read_to_string;
use std::cmp::{max, min};

fn main() {
    let file_data = read_to_string("src/bin/inputs/day6.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    /* Part 1 */
    let times: Vec<f32> = lines[0]
        .split(":").collect::<Vec<&str>>()
        .get(1).unwrap()
        .split(" ")
        .filter(|x| x.trim() != "")
        .map(|x| x.parse::<f32>().unwrap()).collect();
    let distances: Vec<f32> = lines[1]
        .split(":").collect::<Vec<&str>>()
        .get(1).unwrap()
        .split(" ")
        .filter(|x| x.trim() != "")
        .map(|x| x.parse::<f32>().unwrap()).collect();

    let mut total = 1;
    for (t, x) in times.iter().zip(distances.iter()) {
        if t*t - 4.0 * x < 0.0 { continue; }

        let interval: (i32, i32) = (
            max(((t - (t*t - 4.0*(x + 1.0)).powf(0.5)) / 2.0).ceil() as i32, 1),
            min(((t + (t*t - 4.0*(x + 1.0)).powf(0.5)) / 2.0).floor() as i32, *t as i32));

        total *= min(interval.1, t.ceil() as i32) - max(interval.0, 1) + 1;
    }
    println!("Part 1: {total}");

    /* Part 2 */
    let t = lines[0]
        .split(":").collect::<Vec<&str>>()
        .get(1).unwrap()
        .replace(" ", "").parse::<f64>().unwrap();
    let x = lines[1]
        .split(":").collect::<Vec<&str>>()
        .get(1).unwrap()
        .replace(" ", "").parse::<f64>().unwrap();
    let interval: (i64, i64) = (
        max(((t - (t*t - 4.0*(x + 1.0)).powf(0.5)) / 2.0).ceil() as i64, 1),
        min(((t + (t*t - 4.0*(x + 1.0)).powf(0.5)) / 2.0).floor() as i64, t as i64));
    let total = min(interval.1, t.ceil() as i64) - max(interval.0, 1) + 1;
    println!("Part 2: {total}");
}