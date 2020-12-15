use regex::Regex;

use crate::utils::read_lines_true;

pub fn part_1() {
    let lines = read_lines_true("input/day_4.txt").unwrap();

    let pass_list: Vec<Vec<&str>> = lines
        .split(|line| line == "")
        .map(|pass_lines| {
            pass_lines.iter().fold(Vec::new(), |mut vec, line| {
                vec.append(&mut line.split(' ').collect());
                vec
            })
        })
        .collect();

    let cid_re = Regex::new(r"^cid:\d+$").unwrap();

    let result = pass_list
        .iter()
        .filter(|pass| {
            let field_count = pass.iter().count();
            field_count == 8
                || (field_count == 7 && pass.iter().find(|field| cid_re.is_match(field)).is_none())
        })
        .count();
    println!("The number of valid passports: {}", result);
}
