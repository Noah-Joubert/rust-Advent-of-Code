use std::fs::read_to_string;
use std::collections::HashMap;
use num::integer::lcm;

fn main() {
    let file_data = read_to_string("src/bin/inputs/day8.txt").unwrap();
    let lines: Vec<&str> = file_data.split("\n").collect();

    let mut node: String = "AAA".to_string();
    let route: Vec<char> = lines[0].replace(' ', "").chars().collect();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    for line in lines.get(2..).unwrap() {
        let _source_destination: Vec<&str> = line.trim().split('=').collect();
        let source: String = _source_destination[0].trim().to_string();
        let destinations: Vec<String> = _source_destination[1]
            .to_string()
            .replace(&['(', ')'], "")
            .split(',')
            .map(|c| c.trim().to_string())
            .collect();
        network.insert(source.clone(), (destinations[0].clone(), destinations[1].clone()));
    }

    /* Part 1 */
    let mut steps: i64 = 0;
    let mut route_index: usize = 0;
    while node != "ZZZ" {
        if route[route_index] == 'L' {
            node = network.get(&node).unwrap().0.clone();
        } else if route[route_index] == 'R' {
            node = network.get(&node).unwrap().1.clone();
        } else {
            panic!("Unknown route character");
        }

        route_index = (route_index + 1) % route.len();
        steps += 1;
    }
    println!("Part 1: {steps}");

    /* Part 2 */
    let mut result: i64 = 1;
    for (source, _) in &network {
        if source.chars().last().unwrap() != 'A' { continue; }

        let mut node = &source.clone();
        let mut steps: i64 = 0;
        let mut route_index: usize = 0;
        while node.chars().last().unwrap() != 'Z' {
            if route[route_index]  == 'L' {
                node = &network.get(node).unwrap().0;
            } else if route[route_index] == 'R' {
                node = &network.get(node).unwrap().1;
            } else {
                panic!("Unknown route character");
            }

            route_index = (route_index + 1) % route.len();
            steps += 1;
        }
        result = lcm(result, steps);
    }
    println!("Part 2: {result}");
}