use crate::utils::read_lines;

fn find_two(numbers: &[u32]) -> Option<u32> {
    for &n_1 in numbers {
        for &n_2 in numbers {
            if n_1 + n_2 == 2020 {
                return Some(n_1 * n_2);
            }
        }
    }
    None
}

fn find_three(numbers: &[u32]) -> Option<u32> {
    for &n_1 in numbers {
        for &n_2 in numbers {
            for &n_3 in numbers {
                if n_1 + n_2 + n_3 == 2020 {
                    return Some(n_1 * n_2 * n_3);
                }
            }
        }
    }
    None
}

pub fn part_1() {
    let numbers: Vec<u32> = read_lines("input/day_1.txt")
        .unwrap()
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
    if let Some(result) = find_two(&numbers) {
        println!("The result is {}!", result);
    }
}

pub fn part_2() {
    let numbers: Vec<u32> = read_lines("input/day_1.txt")
        .unwrap()
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    if let Some(result) = find_three(&numbers) {
        println!("The result is {}!", result);
    }
}
