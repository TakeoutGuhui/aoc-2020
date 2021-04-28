use regex::Regex;

use crate::utils::read_lines;

/*
You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:

    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.

Your job is to count the passports where all required fields are both present and valid according to the above rules. Here are some example values:
*/
#[allow(dead_code)]
struct Fields {
    byr: u16,
    iyr: u16,
    eyr: u16,
}

#[allow(dead_code)]
fn validate_fields() -> bool {
    true
}

pub fn part_1() {
    let lines = read_lines("input/day_4.txt").unwrap();

    let pass_list: Vec<Vec<&str>> = lines
        .split(|line| line == "")
        .map(|pass_lines| {
            pass_lines.iter().fold(Vec::new(), |mut vec, line| {
                vec.append(&mut line.split(' ').collect());
                vec
            })
        })
        .collect();
    //    println!("{:#?}", pass_list);
    let cid_re = Regex::new(r"^cid:\d+$").unwrap();

    let result = pass_list
        .iter()
        .filter(|pass| {
            let field_count = pass.iter().count();
            field_count == 8
                || (field_count == 7
                    && pass.iter().find(|&&field| cid_re.is_match(field)).is_none())
        })
        .count();
    println!("The number of valid passports: {}", result);
}
