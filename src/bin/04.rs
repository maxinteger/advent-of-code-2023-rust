use itertools::Itertools;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|line| {
                let segments = line
                    .split(": ")
                    .last()
                    .unwrap()
                    .split(" | ")
                    .map(|segment| {
                        segment
                            .split(" ")
                            .filter(|p| !p.is_empty())
                            .map(|p| p.parse::<u32>().unwrap())
                            .sorted()
                            .collect::<Vec<u32>>()
                    })
                    .collect::<Vec<Vec<u32>>>();

                let winning = segments.first().unwrap();
                let owned = segments.last().unwrap();

                let counter = winning.iter().filter(|p| owned.clone().contains(p)).count() as i32;

                if counter > 0 {
                    2_u32.pow((counter - 1) as u32)
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(idx, line)| {
            let segments = line
                .split(": ")
                .last()
                .unwrap()
                .split(" | ")
                .map(|segment| {
                    segment
                        .split(" ")
                        .filter(|p| !p.is_empty())
                        .map(|p| p.parse::<u32>().unwrap())
                        .sorted()
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();

            let winning = segments.first().unwrap();
            let owned = segments.last().unwrap();

            let winning_nums = winning.iter().filter(|p| owned.clone().contains(p)).count() as i32;

            (idx, winning_nums, 1)
        })
        .collect::<Vec<_>>();

    for i in 0..cards.len() {
        let (idx, winning_nums, counter) = cards[i];
        let last_idx = cards.len() - 1;

        if idx == last_idx {
            continue;
        }

        let from = usize::min(idx + 1, last_idx);
        let to = usize::min(idx + winning_nums as usize, last_idx);

        for j in from..=to {
            let (jdx, wn, c) = cards[j];
            cards[j] = (jdx, wn, c + counter);
        }
    }

    Some(
        cards
            .clone()
            .into_iter()
            .map(|(_, _, counter)| counter as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}

/*
1
2
4
4
2
1
*/
