use itertools::Itertools;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .map(|line| {
                let first = line.chars().find(|c| c.is_numeric()).unwrap_or(' ');
                let last = line.chars().rfind(|c| c.is_numeric()).unwrap_or(' ');

                // println!("{} {} {}", line, first, last);

                vec![first, last]
                    .into_iter()
                    .collect::<String>()
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(0)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    Some(
        input
            .split("\n")
            .map(|line| {
                let mut first: u32 = 0;
                let mut last: u32 = 0;
                for idx in 0..line.len() {
                    let res = nums.iter().find_position(|num| {
                        idx + num.len() <= line.len() && line[idx..idx + num.len()] == ***num
                    });
                    if let Some(pos) = res {
                        first = ((pos.0 % 9) + 1) as u32;
                        break;
                    }
                }
                for idx in (0..=line.len()).rev() {
                    let res = nums.iter().find_position(|num| {
                        idx >= num.len() && line[idx - num.len()..idx] == ***num
                    });
                    if let Some(pos) = res {
                        last = ((pos.0 % 9) + 1) as u32;
                        break;
                    }
                }

                // println!("{} {} {}", line, first, last);

                vec![
                    char::from_digit(first, 10).unwrap(),
                    char::from_digit(last, 10).unwrap(),
                ]
                .into_iter()
                .collect::<String>()
                .trim()
                .parse::<u32>()
                .unwrap_or(0)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
