advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let times = lines[0]
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let distances = lines[1]
        .split(":")
        .last()
        .unwrap()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    Some(
        times
            .into_iter()
            .enumerate()
            .map(|(idx, time)| {
                let mut counter = 0;
                for iter in 1..=time {
                    let distance = distances[idx];

                    if distance < (time - iter) * iter {
                        counter += 1;
                    }
                }
                counter
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let time = lines[0]
        .split(":")
        .last()
        .unwrap()
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();

    let distance = lines[1]
        .split(":")
        .last()
        .unwrap()
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();

    let mut counter = 0;
    for iter in 1..=time {
        if distance < (time - iter) * iter {
            counter += 1;
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
