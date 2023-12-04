use itertools::Itertools;
use parse_display::{Display, FromStr};

advent_of_code::solution!(2);
#[derive(Display, FromStr, PartialEq, Debug)]
enum Cubes {
    #[display("{0} red")]
    Red(u32),
    #[display("{0} green")]
    Green(u32),
    #[display("{0} blue")]
    Blue(u32),
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split("\n")
            .enumerate()
            .map(|(idx, line)| {
                let cubes = line
                    .split(": ")
                    .last()
                    .unwrap()
                    .split("; ")
                    .map(|sub| sub.split(", "))
                    .flatten()
                    .filter(|item| !item.is_empty())
                    .map(|item| item.parse::<Cubes>().unwrap())
                    .collect::<Vec<_>>();

                let is_possible = cubes.into_iter().all(|item| match item {
                    Cubes::Red(r) => r <= 12,
                    Cubes::Green(g) => g <= 13,
                    Cubes::Blue(b) => b <= 14,
                });

                // if is_possible {
                //     println!("{} {}", idx, line);
                // }

                (idx + 1, is_possible)
            })
            .filter(|(_, is_possible)| *is_possible)
            .map(|(idx, _)| idx as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim()
            .split("\n")
            .enumerate()
            .map(|(idx, line)| {
                let cubes = line
                    .split(": ")
                    .last()
                    .unwrap()
                    .split("; ")
                    .map(|sub| sub.split(", "))
                    .flatten()
                    .filter(|item| !item.is_empty())
                    .map(|item| item.parse::<Cubes>().unwrap())
                    .collect::<Vec<_>>();

                let r = cubes.iter().fold(u32::MIN, |x, c| {
                    if let Cubes::Red(a) = c {
                        if *a > x {
                            *a
                        } else {
                            x
                        }
                    } else {
                        x
                    }
                });
                let g = cubes.iter().fold(u32::MIN, |x, c| {
                    if let Cubes::Green(a) = c {
                        if *a > x {
                            *a
                        } else {
                            x
                        }
                    } else {
                        x
                    }
                });
                let b = cubes.iter().fold(u32::MIN, |x, c| {
                    if let Cubes::Blue(a) = c {
                        if *a > x {
                            *a
                        } else {
                            x
                        }
                    } else {
                        x
                    }
                });

                // println!("{} {} {} {}", idx, r, g, b);

                r * g * b
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
