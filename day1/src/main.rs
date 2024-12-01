// 1320851
// 26859182

fn main() {
    println!("Hello, world!");
}

pub fn part1(input: &str) -> i32 {
    let mut lefts = vec![];
    let mut rights = vec![];

    input.split("\n").for_each(|line| {
        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        lefts.push(nums[0]);
        rights.push(nums[1]);
    });

    lefts.sort();
    rights.sort();

    let mut i = 0;
    let mut numbers = vec![];

    for right in rights {
        numbers.push((right - lefts[i]).abs());
        i += 1;
    }

    numbers.iter().fold(0, |acc, value| acc + value)
}

pub fn part2(input: &str) -> i32 {
    let left_values: Vec<i32> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .next()
                .expect("Each line must have at least one number")
                .parse()
                .expect("Invalid integer in left column")
        })
        .collect();

    let right_values: Vec<i32> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(1)
                .expect("Each line must have a second number")
                .parse()
                .expect("Invalid integer in right column")
        })
        .collect();

    left_values
        .iter()
        .map(|&left| {
            let matching_rights =
                right_values.iter().filter(|&&right| right == left).count() as i32;
            matching_rights * left
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use std::fs;

    const INPUT: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 11);
    }

    #[test]
    fn process_part1() {
        let input = fs::read_to_string("input.txt").unwrap();
        println!("Result: {}", part1(&input));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 31);
    }

    #[test]
    fn process_part2() {
        let input = fs::read_to_string("input.txt").unwrap();
        println!("Result: {}", part2(&input));
    }
}
