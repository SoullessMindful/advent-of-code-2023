use std::env::args;
use std::fs::read_to_string;

fn main() {
    let clargs: Vec<String> = args().collect();

    let filepath = &clargs[1];

    let lines: Vec<String> = read_to_string(filepath) 
        .unwrap()
        .split("\n\n")
        .map(String::from)
        .collect();

    let seeds = extract_seeds(lines[0].clone());

    let seed_to_soil = extract_rules(lines[1].clone());
    let soil_to_fertilizer = extract_rules(lines[2].clone());
    let fertilizer_to_water = extract_rules(lines[3].clone());
    let water_to_light = extract_rules(lines[4].clone());
    let light_to_temperature = extract_rules(lines[5].clone());
    let temperature_to_humidity = extract_rules(lines[6].clone());
    let humidity_to_location = extract_rules(lines[7].clone());

    let locations: Vec<i64> = seeds.into_iter()
        .map(|s| apply_rules(s, seed_to_soil.clone()))
        .map(|s| apply_rules(s, soil_to_fertilizer.clone()))
        .map(|s| apply_rules(s, fertilizer_to_water.clone()))
        .map(|s| apply_rules(s, water_to_light.clone()))
        .map(|s| apply_rules(s, light_to_temperature.clone()))
        .map(|s| apply_rules(s, temperature_to_humidity.clone()))
        .map(|s| apply_rules(s, humidity_to_location.clone())).collect::<Vec<i64>>();

    println!("{:?}", locations.iter().min().unwrap());
}

fn extract_seeds(line: String) -> Vec<i64> {
    let (_, seeds_string) = line.split_once(" ").unwrap();

    let seeds: Vec<i64> = seeds_string
        .split(" ")
        .map(String::from)
        .map(|x| x.parse::<i64>())
        .map(|x| x.unwrap())
        .collect();

    return seeds;
}

fn extract_rules(block: String) -> Vec<Rule> {
    let (_, rule_strings) = block.split_once("\n").unwrap();

    let rules: Vec<Rule> = rule_strings
        .split("\n")
        .filter(|&x| !x.is_empty())
        .map(String::from)
        .map(parse_rule)
        .collect();

    return rules;
}

fn parse_rule(line: String) -> Rule {
    let components: Vec<String> = line
        .split(" ")
        .map(String::from)
        .collect();

    return Rule {
        target_start: components[0].parse::<i64>().unwrap(),
        source_start: components[1].parse::<i64>().unwrap(),
        range: components[2].parse::<i64>().unwrap(),
    };
}

fn apply_rules(source: i64, rules: Vec<Rule>) -> i64 {
    let rule = rules.into_iter()
        .find(|r| r.source_start <= source && source < r.source_start + r.range);

    match rule {
        Some(r) => return r.target_start + (source - r.source_start),
        None => return source,
    }
}

#[derive(Clone)]
struct Rule {
    source_start: i64,
    target_start: i64,
    range: i64,
}
