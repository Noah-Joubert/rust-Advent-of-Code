use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

struct Workflow {
    name: String,
    rules: Vec<Rule>
}
struct Rule {
    trivial: bool,
    category: char,
    relation: Ordering,
    limit: usize,
    destination: String
}

fn recur(ranges: HashMap<char, (usize, usize)>, workflows: &HashMap<String, crate::Workflow>, wf_string: &str, rule_index: usize) -> i128 {
    let wf = workflows.get(wf_string).unwrap();
    let rule = wf.rules.get(rule_index).unwrap();
    let (condition_met, met_ranges) =
        if rule.trivial {
            (true, ranges.clone())
        } else {
            let mut new_ranges = ranges.clone();
            let mut range = new_ranges[&rule.category];
            if rule.relation == Ordering::Less {
                range = (range.0, min(range.1, rule.limit - 1));
            } else {
                range = (max(range.0, rule.limit + 1), range.1);
            }
            new_ranges.insert(rule.category, range);
            (range.0 <= range.1, new_ranges)
        };
    let (condition_skipped, skipped_ranges) =
        if rule.trivial {
            (false, ranges.clone())
        } else {
            let mut new_ranges = ranges.clone();
            let mut range = new_ranges[&rule.category];
            if rule.relation == Ordering::Greater {
                range = (range.0, min(range.1, rule.limit));
            } else {
                range = (max(range.0, rule.limit), range.1);
            }
            new_ranges.insert(rule.category, range);
            (range.0 <= range.1, new_ranges)
        };

    // trivial rule, or we meet the limit
    return
        if condition_met {
            if rule.destination == "A" {
                let mut total: i128 = 1;
                for (_, limit) in met_ranges {
                    total *= (limit.1 - limit.0) as i128 + 1;
                }
                total
            } else if rule.destination == "R" {
                0
            } else {
                recur(met_ranges, workflows, rule.destination.as_str(), 0)
            }
        } else {
            0
        } +
        if condition_skipped {
            recur(skipped_ranges, workflows, wf_string, rule_index + 1)
        } else {
            0
        };
}

fn main() {
    let file_data = read_to_string("src/bin/inputs/day19.txt").unwrap();

    /* Part 1 */
    let start = Instant::now();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let first_workflow_str = "in";
    let __workflow_strings: Vec<&str> = file_data.split('@').nth(0).unwrap().split('\n').collect();

    // build the workflows
    for s in __workflow_strings {
        let __split: Vec<&str> = s.split('{').collect();
        let name =  __split[0];
        let rule_str = __split[1];
        let split_rule_str: Vec<&str> = rule_str.get(..(rule_str.len() - 1)).unwrap().split(',').collect();

        // build the rules
        let mut rules = vec![];
        for rule in split_rule_str {
            if rule == "A" {
                rules.push(Rule {trivial: true, category: ' ', relation: Ordering::Less, limit: 0, destination: "A".to_string()})
            } else if rule == "R" {
                rules.push(Rule {trivial: true, category: ' ', relation: Ordering::Less, limit: 0, destination: "R".to_string()})
            } else if !rule.contains(':') {
                rules.push(Rule {trivial: true, category: ' ', relation: Ordering::Less, limit: 0, destination: rule.to_string()})
            } else {
                let relation_char =
                    if rule.contains('<') {'<'}
                    else if rule.contains('>') {'>'}
                    else if rule.contains('=') {'='}
                    else {panic!("Unknown relation")};
                let relation =
                    if relation_char == '<' {Ordering::Less}
                    else if relation_char == '>' {Ordering::Greater}
                    else if relation_char == '=' {Ordering::Equal}
                    else {panic!("Unknown relation")};
                let __split: Vec<&str> = rule.split(relation_char).collect();
                let category = __split[0].parse::<char>().unwrap();
                let ___split: Vec<&str> = __split[1].split(':').collect();
                let limit = ___split[0].parse::<usize>().unwrap();
                let destination = ___split[1];
                rules.push(Rule {trivial: false, category: category, relation, limit, destination: destination.to_string()})
            }
        }

        workflows.insert(name.to_string(), Workflow{name: name.to_string(), rules});
    }

    // run each machine part
    let mut total = 0;
    let __machine_parts_str: Vec<&str> = file_data.split('@').nth(1).unwrap().split('\n').collect();
    for part in __machine_parts_str {
        if part == "" { continue; }
        let __category_str: Vec<&str> = part.get(1..(part.len() - 1)).unwrap().split(',').collect();

        // build the part map
        let mut part_map: HashMap<char, usize> = HashMap::new();
        for category in __category_str {
            let category_char: char = category.split('=').nth(0).unwrap().parse::<char>().unwrap();
            let value: usize = category.split('=').nth(1).unwrap().parse::<usize>().unwrap();
            part_map.insert(category_char, value);
        }

        let mut wf = workflows.get(first_workflow_str).unwrap();
        loop {
            let mut break_flag = false;
            // loop through each rule
            for rule in &wf.rules {
                // trivial rule?
                if rule.trivial || part_map.get(&rule.category).unwrap().cmp(&rule.limit) == rule.relation {
                    if rule.destination == "A" {
                        break_flag = true;
                        for (_, i) in &part_map {
                            total += i;
                        }
                    } else if rule.destination == "R" {
                        break_flag = true;
                    } else {
                        wf = workflows.get(rule.destination.as_str()).unwrap();
                    }
                    break;
                }
            }
            if break_flag { break; }
        }
    }

    let end = Instant::now();
    println!("Part 1: {total} in {:?}", end - start);

    /* Part 2 */
    let start = Instant::now();
    let mut limits: HashMap<char, (usize, usize)> = HashMap::new();
    limits.insert('x', (1, 4000));
    limits.insert('m', (1, 4000));
    limits.insert('a', (1, 4000));
    limits.insert('s', (1, 4000));
    let total = recur(limits, &workflows, first_workflow_str, 0);
    let end = Instant::now();
    println!("Part 2: {total} in {:?}", end - start);
}