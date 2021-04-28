use std::collections::HashMap;

use regex::Regex;

use crate::utils::read_lines;

fn parse_line<'a>(h_map: &mut HashMap<&'a str, Option<Vec<(&'a str, u8)>>>, line: &'a str) {
    lazy_static! {
        static ref BAG_RE: Regex = Regex::new(r"([a-z]+ [a-z]+) bags contain").unwrap();
        static ref BAGS_RE: Regex = Regex::new(r"(\d+) ([a-z]+ [a-z]+) bags?").unwrap();
    }
    let cap = BAG_RE.captures(line).unwrap();
    let color = cap.get(1).unwrap().as_str();

    if line.contains("no other bags") {
        h_map.insert(color, None);
    } else {
        let mut bags: Vec<(&str, u8)> = Vec::new();
        for cap in BAGS_RE.captures_iter(line) {
            let clr = cap.get(2).unwrap().as_str();
            let num: u8 = cap.get(1).unwrap().as_str().parse().unwrap();
            bags.push((clr, num));
        }
        h_map.insert(color, Some(bags));
    }
}

fn find_gold(color: &str, h_map: &HashMap<&str, Option<Vec<(&str, u8)>>>) -> bool {
    if color == "shiny gold" {
        true
    } else if let Some(Some(list)) = h_map.get(color) {
        list.iter().any(|(clr, _)| find_gold(clr, h_map))
    } else {
        false
    }
}

fn count_bags(color: &str, h_map: &HashMap<&str, Option<Vec<(&str, u8)>>>) -> u32 {
    if let Some(Some(list)) = h_map.get(color) {
        list.iter()
            .map(|(clr, num)| *num as u32 * count_bags(clr, h_map))
            .sum::<u32>()
            + 1
    } else {
        1
    }
}

pub fn part_1_and_2() {
    let mut h_map: HashMap<&str, Option<Vec<(&str, u8)>>> = HashMap::new();
    let lines = read_lines("input/day_7.txt").unwrap();

    for line in &lines {
        parse_line(&mut h_map, line);
    }

    let result_1 = h_map.keys().filter(|key| find_gold(key, &h_map)).count() - 1;
    println!("Part 1 result is {}", result_1);
    let result_2 = count_bags("shiny gold", &h_map) - 1;
    println!("Part 2 result is {}", result_2);
}
