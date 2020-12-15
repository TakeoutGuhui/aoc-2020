use crate::utils::read_lines_true;

fn check_slope(slope: &[String], right: usize, down: usize, width: usize) -> u32 {
    let mut x = right;
    let mut y = down;

    let mut trees_hit = 0;

    while let Some(line) = slope.get(y) {
        if line.chars().nth(x).unwrap() == '#' {
            trees_hit += 1;
        }
        x = (x + right) % width;
        y += down;
    }
    trees_hit
}

pub fn part_1() {
    let lines = read_lines_true("input/day_3.txt").unwrap();
    let width = lines.get(0).unwrap().chars().count();
    let result = check_slope(&lines, 3, 1, width);
    println!("{} trees hit!", result);
}

pub fn part_2() {
    let lines = read_lines_true("input/day_3.txt").unwrap();
    let width = lines.get(0).unwrap().chars().count();
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result = slopes.iter().fold(1, |acc, slope| {
        acc * check_slope(&lines, slope.0, slope.1, width)
    });

    println!("Part 2: {} trees hit!", result)
}
