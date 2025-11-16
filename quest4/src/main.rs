use anyhow::anyhow;
use std::path::Path;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, multispace0, multispace1};
use winnow::combinator::{alt, eof, separated, separated_pair, terminated};

fn main() -> anyhow::Result<()> {
    let input = read_input("everybody_codes_e2025_q04_p1.txt")?;
    let result = part1(&input)?;
    println!("{result}");

    let input = read_input("everybody_codes_e2025_q04_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");

    let input = read_input("everybody_codes_e2025_q04_p3.txt")?;
    let result = part3(&input)?;
    println!("{result}");

    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let result = std::fs::read_to_string(path)?;
    Ok(result)
}

fn parse_input_data(input: &mut &str) -> ModalResult<Vec<f64>> {
    terminated(separated(1.., parse_f64, multispace1), (multispace0, eof)).parse_next(input)
}

fn parse_f64(input: &mut &str) -> ModalResult<f64> {
    digit1.parse_to::<f64>().parse_next(input)
}

fn parse_pair_list(input: &mut &str) -> ModalResult<Vec<(f64, f64)>> {
    terminated(separated(1.., parse_pair, multispace1), (multispace0, eof)).parse_next(input)
}

fn parse_pair(input: &mut &str) -> ModalResult<(f64, f64)> {
    alt((
        separated_pair(parse_f64, '|', parse_f64),
        parse_f64.map(|x| (x, x)),
    ))
    .parse_next(input)
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let mut inp = input;
    let values = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    let x = values
        .windows(2)
        .map(|w| w[0] / w[1])
        .fold(1.0f64, |x, y| x * y)
        * 2025.0f64;
    Ok(x.floor() as i32)
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let mut inp = input;
    let values = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    let x = values
        .windows(2)
        .map(|w| w[0] / w[1])
        .fold(1.0f64, |x, y| x * y);
    Ok((10000000000000.0f64 / x).ceil() as i64)
}

fn part3(input: &str) -> anyhow::Result<i64> {
    let mut inp = input;
    let values = parse_pair_list(&mut inp).map_err(|e| anyhow!("{e}"))?;
    let x = values
        .windows(2)
        .map(|w| w[0].1 / w[1].0)
        .fold(1.0f64, |x, y| x * y)
        * 100.0f64;
    Ok(x.floor() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"102
75
50
35
13"#;
        let result = part1(input).unwrap();
        assert_eq!(result, 15888);
    }

    #[test]
    fn test_part2() {
        let input = r#"102
75
50
35
13"#;
        let result = part2(input).unwrap();
        assert_eq!(result, 1274509803922);
    }

    #[test]
    fn test_part3() {
        let input = r#"5
7|21
18|36
27|27
10|50
10|50
11"#;
        let result = part3(input).unwrap();
        assert_eq!(result, 6818);
    }
}
