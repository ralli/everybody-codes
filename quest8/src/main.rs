use std::fs::read_to_string;
use std::path::Path;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{eof, separated, terminated};

fn main() -> anyhow::Result<()> {
    let input = read_to_string("everybody_codes_e2025_q08_p1.txt")?;
    let result = part1(&input)?;
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
    let mut inp = input;
    let input_data = terminated(parse_int_list, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|err| anyhow::anyhow!(err))?;
    let max_number = input_data.iter().max().copied().unwrap_or_default();
    let mut result = 0;

    for (i, w1) in input_data
        .as_slice()
        .windows(2).enumerate() {
        for w2 in input_data
            .as_slice()
            .windows(2).skip(i+1) {
            if overlaps((w1[0], w1[1]), (w2[0], w2[1]), max_number) {
                result += 1;
            }
        }
    }
    Ok(result)
}

fn overlaps((p1, p2): (i32, i32), (p3, p4): (i32, i32), max_num: i32) -> bool {
    let (p1, p2) = (p1.min(p2), p1.max(p2));
    let (p3, p4) = (p3.min(p4), p3.max(p4));

    let (i1, i2) = (p1, p2);
    let (i3, i4) = (p2, p1 + max_num);

    (i1 < p3 && i2 > p3 && i3 < p4 && i4 > p4) || (i1 < p4 && i2 > p4 && i3 < p3 && i4 > p4)
}

fn parse_int_list(input: &mut &str) -> ModalResult<Vec<i32>> {
    separated(1.., parse_int, ',').parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<i32> {
    digit1.parse_to::<i32>().parse_next(input)
}

fn read_file(filename: impl AsRef<Path>) -> anyhow::Result<String> {
    let result = read_to_string(filename)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps() {
        // assert!(!overlaps((1,5), (2,5), 8));
        // assert!(overlaps((1,5), (2,6), 8));
        assert!(overlaps((8,4), (1,5), 8));
        assert!(overlaps((8,4), (2,6), 8));
    }
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
}
