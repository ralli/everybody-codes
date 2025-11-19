use std::collections::BTreeMap;
use std::fs::read_to_string;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{eof, separated, terminated};

fn main() -> anyhow::Result<()> {
    let input = read_to_string("everybody_codes_e2025_q08_p1.txt")?;
    let result = part1(&input)?;
    println!("{result}");

    let input = read_to_string("everybody_codes_e2025_q08_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");

    let input = read_to_string("everybody_codes_e2025_q08_p3.txt")?;
    let result = part3(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let input_data = terminated(parse_int_list, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|err| anyhow::anyhow!(err))?;
    let max_number = input_data.iter().max().copied().unwrap_or_default();
    let want = (max_number - 2) / 2 + 1;
    let result = input_data
        .as_slice()
        .windows(2)
        .filter(|w| (w[1] - w[0]).abs() == want)
        .count();
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut result = 0;
    let mut inp = input;
    let input_data = terminated(parse_int_list, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|err| anyhow::anyhow!(err))?;
    let pairs: Vec<(i32, i32)> = input_data
        .as_slice()
        .windows(2)
        .map(|w| (w[0].min(w[1]), w[0].max(w[1])))
        .collect();

    for (i, (x, y)) in pairs.iter().enumerate() {
        for (a, b) in pairs.iter().skip(i + 1) {
            if overlaps(*a, *b, *x, *y) {
                result += 1;
            }
        }
    }

    Ok(result)
}

pub fn overlaps(a: i32, b: i32, x: i32, y: i32) -> bool {
    // strings with equal endpoints do not overlap
    if a == x || a == y || b == x || b == y {
        return false;
    }
    // one point is within the interval of the other.
    // one point is not within the interval of the other.
    (a < x && x < b) != (a < y && y < b)
}

pub fn part3(input: &str) -> anyhow::Result<i32> {
    let mut inp = input;
    let input_data = terminated(parse_int_list, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|err| anyhow::anyhow!(err))?;
    let pairs: Vec<(i32, i32)> = input_data
        .as_slice()
        .windows(2)
        .map(|w| (w[0].min(w[1]), w[0].max(w[1])))
        .collect();
    let mut counts: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    let max_num = input_data.iter().max().copied().unwrap_or_default();
    for (x, y) in pairs.iter() {
        for a in 1..=max_num {
            for b in a + 1..=max_num {
                if overlaps(a, b, *x, *y) {
                    *counts.entry((a, b)).or_default() += 1;
                }
            }
        }
    }

    Ok(counts.values().max().copied().unwrap_or_default())
}

fn parse_int_list(input: &mut &str) -> ModalResult<Vec<i32>> {
    separated(1.., parse_int, ',').parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<i32> {
    digit1.parse_to::<i32>().parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1,5,2,6,8,4,1,7,3";
        let result = part1(input).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2() {
        let input = "1,5,2,6,8,4,1,7,3,5,7,8,2";
        let result = part2(input).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part3() {
        let input = "1,5,2,6,8,4,1,7,3,6";
        let result = part3(input).unwrap();
        assert_eq!(result, 6);
    }
}
