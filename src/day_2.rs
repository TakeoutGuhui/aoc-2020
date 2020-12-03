use regex::Regex;

use crate::utils::read_lines_true;

#[derive(Debug)]
struct PassCheck<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

impl PassCheck<'_> {
    fn check_password(&self) -> bool {
        let count = self.password.matches(self.letter).count();
        count >= self.min && count <= self.max
    }

    fn check_password_v2(&self) -> bool {
        let first = self.password.chars().nth(self.min - 1);
        let second = self.password.chars().nth(self.max - 1);

        (first.unwrap() == self.letter) ^ (second.unwrap() == self.letter)
    }
}

fn parse_line<'a>(re: &Regex, line: &'a str) -> Option<PassCheck<'a>> {
    if let Some(cap) = re.captures(line) {
        let pass_check = PassCheck {
            min: cap.get(1).unwrap().as_str().parse().unwrap(),
            max: cap.get(2).unwrap().as_str().parse().unwrap(),
            letter: cap.get(3).unwrap().as_str().chars().nth(0).unwrap(),
            password: cap.get(4).unwrap().as_str(),
        };
        return Some(pass_check);
    }
    None
}

pub fn part_1() {
    let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)$").unwrap();

    let correct_count = read_lines_true("input/day_2.txt")
        .expect("couldn't read")
        .iter()
        .map(|line| parse_line(&re, line).unwrap())
        .filter(|pass| pass.check_password())
        .count();

    println!("Correct passwords: {}", correct_count);
}

pub fn part_2() {
    let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)$").unwrap();

    let correct_count = read_lines_true("input/day_2.txt")
        .expect("couldn't read")
        .iter()
        .map(|line| parse_line(&re, line).unwrap())
        .filter(|pass| pass.check_password_v2())
        .count();

    println!("Correct passwords: {}", correct_count);
}
