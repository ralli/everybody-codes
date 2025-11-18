use anyhow::anyhow;
use itertools::Itertools;
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::path::Path;
use winnow::ascii::{alpha1, line_ending, multispace0, multispace1, space0};
use winnow::combinator::{eof, separated, separated_pair, terminated};
use winnow::token::any;
use winnow::{ModalResult, Parser};

macro_rules! full_input {
    ($p: expr) => {
        terminated($p, (multispace0, eof))
    };
}

fn main() -> anyhow::Result<()> {
    let input = read_file("everybody_codes_e2025_q07_p1.txt")?;
    let result = part1(&input)?;
    println!("{result}");

    let input = read_file("everybody_codes_e2025_q07_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");

    let input = read_file("everybody_codes_e2025_q07_p3.txt")?;
    let result = part3(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<String> {
    let mut inp = input;
    let input_data = full_input!(parse_input_data)
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("Failed to parse input data: {}", e))?;
    let result = input_data
        .names
        .into_iter()
        .filter(|name| matches_word(name, &input_data.adj))
        .next()
        .ok_or_else(|| anyhow!("No matching word found"))?;
    Ok(result)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let input_data = full_input!(parse_input_data)
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("Failed to parse input data: {}", e))?;
    let result = input_data
        .names
        .iter()
        .enumerate()
        .filter(|(idx, name)| matches_word(name, &input_data.adj))
        .map(|(idx, _name)| idx + 1)
        .fold(0, |acc, idx| acc + idx);
    Ok(result)
}

fn part3(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let input_data = full_input!(parse_input_data)
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("Failed to parse input data: {}", e))?;
    let names: Vec<String> = input_data
        .names
        .iter()
        .filter(|name| matches_word(name, &input_data.adj))
        .cloned()
        .collect();
    let mut q: VecDeque<String> = VecDeque::from(names);
    let mut result = HashSet::new();

    while let Some(current) = q.pop_front() {
        if current.len() > 11 {
            continue;
        }

        if current.len() >= 7 {
            result.insert(current.to_string());
        }

        if current.len() == 11 {
            continue;
        }

        let c = current.chars().last().unwrap();
        let Some(next) = input_data.adj.get(&c) else {
            continue;
        };
        for next_c in next.iter() {
            q.push_back(format!("{}{}", current, next_c));
        }
    }
    Ok(result.len())
}

fn matches_word(word: &str, adj: &BTreeMap<char, Vec<char>>) -> bool {
    for (c1, c2) in word.chars().tuple_windows::<(char, char)>() {
        let Some(next) = adj.get(&c1) else {
            return false;
        };
        if !next.contains(&c2) {
            return false;
        }
    }
    true
}

#[derive(Debug)]
struct InputData {
    names: Vec<String>,
    adj: BTreeMap<char, Vec<char>>,
}

fn parse_input_data(input: &mut &str) -> ModalResult<InputData> {
    separated_pair(parse_names, multispace1, parse_adj_list)
        .map(|(names, adj)| InputData { names, adj })
        .parse_next(input)
}

fn parse_names(input: &mut &str) -> ModalResult<Vec<String>> {
    separated(1.., alpha1.map(|s: &str| s.to_string()), ',').parse_next(input)
}

fn parse_adj_list(input: &mut &str) -> ModalResult<BTreeMap<char, Vec<char>>> {
    let entries: Vec<(char, Vec<char>)> =
        separated(1.., parse_adj_entry, line_ending).parse_next(input)?;
    let result = BTreeMap::from_iter(entries.into_iter());
    Ok(result)
}

fn parse_adj_entry(input: &mut &str) -> ModalResult<(char, Vec<char>)> {
    separated_pair(parse_alpha, (space0, '>', space0), parse_alpha_list).parse_next(input)
}

fn parse_alpha_list(input: &mut &str) -> ModalResult<Vec<char>> {
    separated(1.., parse_alpha, ',').parse_next(input)
}

fn parse_alpha(input: &mut &str) -> ModalResult<char> {
    any.verify(|c: &char| c.is_alphabetic()).parse_next(input)
}

fn read_file(filename: impl AsRef<Path>) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"Oronris,Urakris,Oroneth,Uraketh

r > a,i,o
i > p,w
n > e,r
o > n,m
k > f,r
a > k
U > r
e > t
O > r
t > h"#;
        let result = part1(input).unwrap();
        assert_eq!(result, "Oroneth");
    }

    #[test]
    fn test_part2() {
        let input = r#"OXanverax,Khargyth,Nexzeth,Helther,Braerex,Tirgryph,Kharverax

r > v,e,a,g,y
a > e,v,x,r
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i"#;
        let result = part2(input).unwrap();
        assert_eq!(result, 23);
    }

    #[test]
    fn test_part3() {
        let input = r#"Xaryt

X > a,o
a > r,t
r > y,e,a
h > a,e,v
t > h
v > e
y > p,t"#;

        let result = part3(input).unwrap();
        assert_eq!(result, 25);
    }

    #[test]
    fn test_part3_2() {
        let input = r#"Khara,Xaryt,Noxer,Kharax

r > v,e,a,g,y
a > e,v,x,r,g
e > r,x,v,t
h > a,e,v
g > r,y
y > p,t
i > v,r
K > h
v > e
B > r
t > h
N > e
p > h
H > e
l > t
z > e
X > a
n > v
x > z
T > i"#;

        let result = part3(input).unwrap();
        assert_eq!(result, 1154);
    }
}
