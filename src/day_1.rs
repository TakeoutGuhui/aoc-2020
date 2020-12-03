use crate::utils::read_lines;

fn find_two(numbers: &Vec<u32>) -> Option<u32> {
    for &n_1 in numbers {
        for &n_2 in numbers {
            if n_1 + n_2 == 2020 {
                return Some(n_1 * n_2);
            }
        }
    }
    None
}

fn find_three(numbers: &Vec<u32>) -> Option<u32> {
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
    if let Ok(numbers) = read_lines("input/day_1.txt") {
        if let Some(result) = find_two(&numbers) {
            println!("The result is {}!", result);
        }
    }
}

pub fn part_2() {
    if let Ok(numbers) = read_lines("input/day_1.txt") {
        if let Some(result) = find_three(&numbers) {
            println!("The result is {}!", result);
        }
    }
}
