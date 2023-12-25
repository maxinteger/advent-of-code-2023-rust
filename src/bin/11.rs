use advent_of_code::utils::ascii_matrix::*;
use itertools::Itertools;
advent_of_code::solution!(11);

const SPACE: char = '.';
const STAR: char = '#';

pub fn part_one(input: &str) -> Option<u32> {
    let mut mat = AsciiMatrix::from_vec_of_vec(
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );

    (0..mat.cols)
        .filter(|col_num| mat.get_col(*col_num).unwrap().iter().all(|c| *c == SPACE))
        .collect::<Vec<usize>>()
        .iter()
        .enumerate()
        .for_each(|(idx, col_num)| mat.insert_col(*col_num + idx, &vec![SPACE; mat.rows]));

    (0..mat.rows)
        .filter(|row_num| mat.get_row(*row_num).unwrap().iter().all(|c| *c == SPACE))
        .collect::<Vec<usize>>()
        .iter()
        .enumerate()
        .for_each(|(idx, row_num)| mat.insert_row(*row_num + idx, &vec![SPACE; mat.cols]));

    // println!("{}", mat.debug(""));

    let starts = mat
        .iter()
        .filter_map(|(coord, c)| if *c == STAR { Some((coord, c)) } else { None })
        .collect::<Vec<_>>();

    Some(
        starts
            .iter()
            .cartesian_product(starts.iter())
            .filter(|(a, b)| a != b)
            .map(|((a, _), (b, _))| a.step_distance(b) as u32)
            .sum::<u32>()
            / 2,
    )
}

pub fn part_two(input: &str) -> Option<u128> {
    const SCALE: usize = 1_000_000;
    let mat = AsciiMatrix::from_vec_of_vec(
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );

    let space_col = (0..mat.cols)
        .filter(|col_num| mat.get_col(*col_num).unwrap().iter().all(|c| *c == SPACE))
        .collect::<Vec<usize>>();

    let space_row = (0..mat.rows)
        .filter(|row_num| mat.get_row(*row_num).unwrap().iter().all(|c| *c == SPACE))
        .collect::<Vec<usize>>();

    // println!("{}", mat.debug(""));

    let starts = mat
        .iter()
        .filter_map(|(coord, c)| if *c == STAR { Some((coord, c)) } else { None })
        .collect::<Vec<_>>();

    Some(
        starts
            .iter()
            .cartesian_product(starts.iter())
            .filter(|(a, b)| a != b)
            .map(|((a, _), (b, _))| {
                let min_x = a.x.min(b.x) as usize;
                let max_x = a.x.max(b.x) as usize;
                let min_y = a.y.min(b.y) as usize;
                let max_y = a.y.max(b.y) as usize;
                let xd = space_col
                    .iter()
                    .filter(|c| **c >= min_x && **c < max_x)
                    .count()
                    * (SCALE - 1);
                let yd = space_row
                    .iter()
                    .filter(|r| **r >= min_y && **r < max_y)
                    .count()
                    * (SCALE - 1);

                ((max_x - min_x) + xd + (max_y - min_y) + yd) as u128
            })
            .sum::<u128>()
            / 2,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
        //assert_eq!(result, Some(1030));
    }
}
