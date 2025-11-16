use anyhow::anyhow;
use std::collections::BTreeMap;
use std::path::Path;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{eof, separated, terminated};

fn main() -> anyhow::Result<()> {
    let input = read_input("everybody_codes_e2025_q03_p1.txt")?;
    let result = part1(&input)?;
    println!("{result}");
    let input = read_input("everybody_codes_e2025_q03_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");
    let input = read_input("everybody_codes_e2025_q03_p3.txt")?;
    let result = part3(&input)?;
    println!("{result}");
    Ok(())
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let result = std::fs::read_to_string(path)?;
    Ok(result)
}

fn parse_input_data(input: &mut &str) -> ModalResult<Vec<i32>> {
    terminated(separated(1.., parse_int, ','), (multispace0, eof)).parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<i32> {
    digit1.parse_to::<i32>().parse_next(input)
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let mut inp = input;
    let mut crates = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    crates.sort_by(|a, b| b.cmp(a));
    crates.dedup();
    Ok(crates.iter().sum())
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let mut inp = input;
    let mut crates = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    crates.sort_by(|a, b| b.cmp(a));
    crates.dedup();
    let size = crates.len();
    let start = size - 20;
    let bla = &crates[start..];
    assert_eq!(bla.len(), 20);
    Ok((crates[start..]).iter().sum())
}

fn part3(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let crates = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    let hist: BTreeMap<i32, usize> = crates.into_iter().fold(BTreeMap::new(), |mut acc, x| {
        let e = acc.entry(x).or_default();
        *e += 1;
        acc
    });
    Ok(hist.values().max().copied().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "10,5,1,10,3,8,5,2,2";
        let result = part1(input).unwrap();
        assert_eq!(result, 29);
    }

    #[test]
    fn test_part2() {
        let input = "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77";
        let result = part2(input).unwrap();
        assert_eq!(result, 781);
    }

    #[test]
    fn test_part3() {
        let input = "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77";
        let result = part3(input).unwrap();
        assert_eq!(result, 3);
    }
}
