use itertools::Itertools;
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut lasts = vec![];
                let mut nums: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|p| p.parse::<i32>().ok())
                    .collect();

                loop {
                    lasts.push(nums.last().unwrap().clone());
                    if nums.iter().all(|n| *n == 0) {
                        break;
                    }
                    nums = nums
                        .into_iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect::<Vec<i32>>();
                }

                lasts.into_iter().rev().reduce(|a, b| a + b).unwrap()
            })
            .sum::<i32>() as u32,
    )
}

// 2043183841
// 26211518
pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut firsts = vec![];
                let mut nums: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|p| p.parse::<i32>().ok())
                    .collect();

                loop {
                    firsts.push(nums.first().unwrap().clone());
                    if nums.iter().all(|n| *n == 0) {
                        break;
                    }
                    nums = nums
                        .into_iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect::<Vec<i32>>();
                }

                firsts.into_iter().rev().reduce(|a, b| b - a).unwrap()
            })
            .sum::<i32>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
