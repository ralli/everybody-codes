use std::collections::HashMap;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let input = read_file("everybody_codes_e2025_q06_p1.txt")?;
    let result = part1(&input)?;
    println!("{result}");

    let input = read_file("everybody_codes_e2025_q06_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");

    let input = read_file("everybody_codes_e2025_q06_p3.txt")?;
    let result = part3(&input, 1000, 1000)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut multipliers: HashMap<char, usize> = HashMap::new();
    let mut result = 0;
    for c in input.chars().rev().filter(|c| *c == 'a' || *c == 'A') {
        if c.is_ascii_lowercase() {
            *multipliers.entry(c).or_insert(0) += 1;
        } else {
            result += multipliers
                .get(&c.to_ascii_lowercase())
                .copied()
                .unwrap_or(0);
        }
    }
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut multipliers: HashMap<char, usize> = HashMap::new();
    let mut result = 0;
    for c in input.chars().rev() {
        if c.is_ascii_lowercase() {
            *multipliers.entry(c).or_insert(0) += 1;
        } else {
            result += multipliers
                .get(&c.to_ascii_lowercase())
                .copied()
                .unwrap_or(0);
        }
    }
    Ok(result)
}

fn part3(input: &str, rep: usize, limit: usize) -> anyhow::Result<usize> {
    let mut result = 0;
    let chars: Vec<char> = input.to_string().chars().collect();
    let chars = Arr::new(chars, rep);
    let size = chars.len();
    for i in 0..size {
        let c = chars.get(i);
        if c.is_ascii_lowercase() {
            let count1 = count_to_left(&chars, i, limit);
            let count2 = count_to_right(&chars, i, limit);
            let count = count1 + count2;
            // println!("count: {c}: {count1} {count2} {count}");
            result += count;
        }
    }
    Ok(result)
}

fn count_to_left(chars: &Arr, start: usize, limit: usize) -> usize {
    let ch = chars.get(start);
    if ch.is_ascii_uppercase() {
        return 0;
    }
    let want = ch.to_ascii_uppercase();
    let mut idx = start;
    let mut count = 0;
    let mut result = 0;
    while idx > 0 && count < limit {
        idx -= 1;
        let curr = chars.get(idx);
        if curr == want {
            result += 1;
        }
        count += 1;
    }
    result
}

fn count_to_right(chars: &Arr, start: usize, limit: usize) -> usize {
    let ch = chars.get(start);
    if ch.is_ascii_uppercase() {
        return 0;
    }
    let size = chars.len();
    let want = ch.to_ascii_uppercase();
    let mut idx = start;
    let mut count = 0;
    let mut result = 0;
    while idx + 1 < size && count < limit {
        idx += 1;
        let curr = chars.get(idx);
        if curr == want {
            result += 1;
        }
        count += 1;
    }
    result
}

fn read_file(filename: impl AsRef<Path>) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content)
}

#[derive(Debug)]
struct Arr {
    chars: Vec<char>,
    rep: usize,
}

impl Arr {
    fn new(chars: Vec<char>, rep: usize) -> Self {
        Self { chars, rep }
    }

    fn len(&self) -> usize {
        self.chars.len() * self.rep
    }

    fn get(&self, idx: usize) -> char {
        self.chars[idx % self.chars.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part3() {
        let input = "AABCBABCABCabcabcABCCBAACBCa";
        let limit = 10;
        let result = part3(input, 1, limit).unwrap();
        assert_eq!(result, 34);
    }

    #[test]
    fn test_part3_2() {
        let input = "AABCBABCABCabcabcABCCBAACBCa";
        let limit = 10;
        let result = part3(input, 2, limit).unwrap();
        assert_eq!(result, 72);
    }

    #[test]
    fn test_count_to_left() {
        let input = Arr::new(
            "AABCBABCABCabcabcABCCBAACBCa"
                .chars()
                .collect::<Vec<char>>(),
            1,
        );
        let start = 11;
        let limit = 10;
        let result = count_to_left(&input, 11, limit);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_count_to_right() {
        let input = Arr::new(
            "AABCBABCABCabcabcABCCBAACBCa"
                .chars()
                .collect::<Vec<char>>(),
            1,
        );
        let start = 11;
        let limit = 10;
        let result = count_to_right(&input, 11, limit);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part1() {
        let input = "ABabACacBCbca";
        let result = part1(input).unwrap();
        assert_eq!(result, 5);
    }
}
