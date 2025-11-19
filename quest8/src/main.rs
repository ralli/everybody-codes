use std::collections::BTreeMap;
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

    let input = read_to_string("everybody_codes_e2025_q08_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");

    let input = read_to_string("everybody_codes_e2025_q08_p3.txt")?;
    let result = part3_2(&input);
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
    let mut result = 0;

    let mut links: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for w in input_data.as_slice().windows(2) {
        let v1 = w[0].min(w[1]);
        let v2 = w[0].max(w[1]);
        links.entry(v1).or_default().push(v2);
    }
    let mut freq: BTreeMap<i32, usize> = BTreeMap::new();

    for (start, ends) in links.iter() {
        for &end in ends {
            for i in start + 1..end {
                result += freq.get(&i).copied().unwrap_or_default();
            }
        }

        for &end in ends {
            let mut e = freq.entry(end).or_default();
            *e += 1;
        }
    }

    Ok(result)
}

pub fn part3_2(notes: &str) -> i32 {
    let mut links = vec![vec![]; 257];
    let mut delta = vec![0; 258];
    let mut result = 0;
    let mut inp = notes;
    let input_data = terminated(parse_int_list, (multispace0, eof))
        .parse_next(&mut inp)
        .unwrap();
    let parsed: Vec<_> = input_data
        .as_slice()
        .windows(2)
        .map(|w| (w[0].min(w[1]) as usize, w[0].max(w[1]) as usize))
        .collect();

    for (start, end) in parsed {
        links[start].push(end);
        delta[start + 1] += 1;
        delta[end] -= 1;
    }

    for start in 1..255 {
        for &end in &links[start] {
            delta[end] += 2;
            delta[end + 1] -= 1;
        }

        for &end in &links[start - 1] {
            delta[end] -= 1;
            delta[end + 1] += 2;
        }

        let mut cuts = 0;

        for &next in &delta[start + 2..257] {
            cuts += next;
            result = result.max(cuts);
        }
    }

    result
}

pub fn part3(input: &str) -> anyhow::Result<i32> {
    let mut inp = input;
    let input_data = terminated(parse_int_list, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|err| anyhow::anyhow!(err))?;
    let mut delta: BTreeMap<i32, i32> = BTreeMap::new();
    let mut result = 0;

    let mut links: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for w in input_data.as_slice().windows(2) {
        let v1 = w[0].min(w[1]);
        let v2 = w[0].max(w[1]);
        links.entry(v1).or_default().push(v2);
        *delta.entry(v1).or_default() += 1;
        *delta.entry(v2).or_default() -= 1;
    }

    let max_value = input_data.iter().max().copied().unwrap_or_default();
    let empty_vec: Vec<i32> = Vec::new();
    for start in 1..max_value {
        let ends = links.get(&start).unwrap_or(&empty_vec);
        for &end in ends.iter() {
            *delta.entry(end).or_default() += 2;
            *delta.entry(end + 1).or_default() -= 1;
        }

        let ends = links.get(&(start - 1)).unwrap_or(&empty_vec);
        for &end in ends.iter() {
            *delta.entry(end).or_default() -= 1;
            *delta.entry(end + 1).or_default() += 2;
        }

        let mut cuts = 0;

        for i in (start + 2..=max_value) {
            let next = delta.get(&i).copied().unwrap_or_default();
            cuts += next;
            result = result.max(cuts);
        }
    }

    Ok(result)
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
        assert_eq!(result, 7);
    }
}
