use advent_of_code::utils::bounding_box::BoundingBox;
advent_of_code::solution!(3);

#[derive(Debug)]
struct Num {
    bb: BoundingBox,
    number: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers = vec![];
    let mut symbols: Vec<BoundingBox> = vec![];

    input.trim().split("\n").enumerate().for_each(|(y, line)| {
        let mut num_from: isize = -1;
        let chars = line.chars();
        let len = chars.clone().count();

        for (x, ch) in chars.enumerate() {
            if ch.is_numeric() && num_from == -1 {
                num_from = x as isize
            }
            if (!ch.is_numeric() || x + 1 == len) && num_from != -1 {
                let end = if x + 1 == len && ch.is_numeric() {
                    x
                } else {
                    x - 1
                };

                let number = line[(num_from as usize)..=(end as usize)]
                    .parse::<u32>()
                    .unwrap();

                numbers.push(Num {
                    bb: BoundingBox::new(num_from as f32, y as f32, x as f32, (y as f32) + 1.0),
                    number,
                });
                num_from = -1;
            }
            if !ch.is_numeric() && ch != '.' {
                symbols.push(BoundingBox::new(
                    x as f32,
                    y as f32,
                    (x as f32) + 1.0,
                    (y as f32) + 1.0,
                ));
            }
        }
    });

    // numbers.iter().clone().for_each(|num| {
    //     if symbols.iter().any(|symbol| symbol.intersects(&num.bb)) {
    //         println!("{}", num.number);
    //     } else {
    //         println!("=> {}", num.number);
    //     }
    // });

    Some(
        numbers
            .into_iter()
            .filter(|num| symbols.iter().any(|symbol| symbol.intersects(&num.bb)))
            .map(|num| num.number)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut numbers = vec![];
    let mut symbols: Vec<BoundingBox> = vec![];

    input.trim().split("\n").enumerate().for_each(|(y, line)| {
        let mut num_from: isize = -1;
        let chars = line.chars();
        let len = chars.clone().count();

        for (x, ch) in chars.enumerate() {
            if ch.is_numeric() && num_from == -1 {
                num_from = x as isize
            }
            if (!ch.is_numeric() || x + 1 == len) && num_from != -1 {
                let end = if x + 1 == len && ch.is_numeric() {
                    x
                } else {
                    x - 1
                };

                let number = line[(num_from as usize)..=(end as usize)]
                    .parse::<u32>()
                    .unwrap();

                numbers.push(Num {
                    bb: BoundingBox::new(num_from as f32, y as f32, x as f32, (y as f32) + 1.0),
                    number,
                });
                num_from = -1;
            }
            if ch == '*' {
                symbols.push(BoundingBox::new(
                    x as f32,
                    y as f32,
                    (x as f32) + 1.0,
                    (y as f32) + 1.0,
                ));
            }
        }
    });

    Some(
        symbols
            .into_iter()
            .map(|symbol| {
                numbers
                    .iter()
                    .filter(|num| symbol.intersects(&num.bb))
                    .map(|num| num.number)
                    .collect::<Vec<_>>()
            })
            .filter(|list| list.len() == 2)
            .map(|list| list[0] * list[1])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
