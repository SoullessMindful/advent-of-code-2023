use std::env::args;
use std::fs::read_to_string;
use std::iter::zip;
use std::cmp::{min,max};

fn main() {
    let clargs: Vec<String> = args().collect();

    let filepath = &clargs[1];

    let lines: Vec<String> = read_to_string(filepath) 
        .unwrap()
        .split("\n\n")
        .map(String::from)
        .collect();

    let seed_ranges = extract_seeds(lines[0].clone());
    let seeds = extract_seeds_from_ranges(seed_ranges);

    let seed_to_soil = extract_rules(lines[1].clone());
    let soil_to_fertilizer = extract_rules(lines[2].clone());
    let fertilizer_to_water = extract_rules(lines[3].clone());
    let water_to_light = extract_rules(lines[4].clone());
    let light_to_temperature = extract_rules(lines[5].clone());
    let temperature_to_humidity = extract_rules(lines[6].clone());
    let humidity_to_location = extract_rules(lines[7].clone());

    let locations: Vec<i64> = seeds.into_iter()
        .flat_map(|s| apply_rules(s, seed_to_soil.clone()))
        .flat_map(|s| apply_rules(s, soil_to_fertilizer.clone()))
        .flat_map(|s| apply_rules(s, fertilizer_to_water.clone()))
        .flat_map(|s| apply_rules(s, water_to_light.clone()))
        .flat_map(|s| apply_rules(s, light_to_temperature.clone()))
        .flat_map(|s| apply_rules(s, temperature_to_humidity.clone()))
        .flat_map(|s| apply_rules(s, humidity_to_location.clone()))
        .map(|s| s.start)
        .collect::<Vec<i64>>();

    println!("{:?}", locations.iter().min().unwrap());
}

fn extract_seeds_from_ranges(seed_ranges: Vec<i64>) -> Vec<SeedRange> {
    let seed_starts: Vec<i64> = seed_ranges.clone().into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, e)| e)
        .collect();

    let seed_range_lengths: Vec<i64> = seed_ranges.into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 1)
        .map(|(_, e)| e)
        .collect();

    return zip(seed_starts, seed_range_lengths)
        .map(|(s, l)| SeedRange {
            start: s,
            length: l,
        })
        .collect();
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

fn apply_rules(source: SeedRange, rules: Vec<Rule>) -> Vec<SeedRange> {
    let mut results = vec![];
    let mut mut_source = source;

    while mut_source.length > 0 {
        let (result, new_source) = apply_rules_rec(mut_source, rules.clone());

        results.push(result);
        mut_source = new_source;
    }

    return results;
}

fn apply_rules_rec(source: SeedRange, rules: Vec<Rule>) -> (SeedRange, SeedRange) {
    match rules.iter().find(|&r| source.start >= r.source_start && source.start < r.source_start + r.range) {
        Some(r) => {
            let end = min(r.source_start + r.range, source.start + source.length);
            let length = end - source.start;
            let result_start = source.start - r.source_start + r.target_start;

            return (
                SeedRange {
                    start: result_start,
                    length: length,
                },
                SeedRange {
                    start: end,
                    length: source.length - end + source.start,
                },
            )
        }
        None => {
            match rules.iter().filter(|&r| r.source_start > source.start).min_by_key(|r| r.source_start) {
                Some (r) => {
                    let end = min(r.source_start, source.start + source.length);
                    let length = end - source.start;

                    return (
                        SeedRange {
                            start: source.start,
                            length: length,
                        },
                        SeedRange {
                            start: end,
                            length: source.length - end + source.start,
                        },
                    )
                }
                None => {
                    return (
                        SeedRange {
                            start: source.start,
                            length: source.length,
                        },
                        SeedRange {
                            start: source.start + source.length,
                            length: 0,
                        },
                    )
                }
            }
        }
    }
}

#[derive(Clone)]
struct Rule {
    source_start: i64,
    target_start: i64,
    range: i64,
}

#[derive(Clone)]
struct SeedRange {
    start: i64,
    length: i64,
}
