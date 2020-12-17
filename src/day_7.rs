use std::collections::HashMap;

use regex::Regex;

use crate::utils::read_lines_true;

fn parse_line<'a>(h_map: &mut HashMap<&'a str, Vec<&'a str>>, line: &'a str, re: &Regex) {
    let mut bags: Vec<&str> = Vec::new();
    let mut cap_iter = re.captures_iter(line);
    let color = cap_iter.next().unwrap().get(1).unwrap().as_str();
    for cap in cap_iter {
        bags.push(cap.get(1).unwrap().as_str());   
    }
    h_map.insert(color, bags);
}

pub fn find_gold(color: &str, h_map: &HashMap<&str, Vec<&str>>) -> bool{
    if color == "shiny gold" {
        return true;
    } else if color == "no other"{
        return false
    } else {
        h_map.get(color).unwrap().iter().any(|clr| find_gold(clr, h_map))
    } 
}

pub fn part_1() {
    let mut h_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let re = Regex::new(r"([a-z]+ [a-z]+) bags?").unwrap();
    let lines = read_lines_true("input/day_7.txt").unwrap();

    for line in &lines {
        parse_line(&mut h_map, line, &re);    
    }
   
    let result = h_map.keys().filter(|key| find_gold(key, &h_map)).count();
    println!("The result is {}", result-1);
}