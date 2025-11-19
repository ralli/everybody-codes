use std::cmp::Ordering;
use std::path::Path;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, multispace0, multispace1};
use winnow::combinator::{eof, separated, terminated};

macro_rules! full_input {
    ($p: expr) => {
        terminated($p, (multispace0, eof))
    };
}

fn main() -> anyhow::Result<()> {
    let input = read_file("everybody_codes_e2025_q05_p1.txt")?;
    let result = part1(&input)?;
    println!("{result}");

    let input = read_file("everybody_codes_e2025_q05_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");

    let input = read_file("everybody_codes_e2025_q05_p3.txt")?;
    let result = part3(&input)?;
    println!("{result}");

    Ok(())
}

fn read_file(file_name: impl AsRef<Path>) -> anyhow::Result<String> {
    let file_name = file_name.as_ref();
    let contents = std::fs::read_to_string(file_name)?;
    Ok(contents)
}

fn part1(input: &str) -> anyhow::Result<i64> {
    let mut inp = input;
    let data = full_input!(parse_sword_data)
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("Failed to parse input data: {}", e))?;
    let fb = Fishbone::from_values(&data.values);
    Ok(fb.quality())
}

fn part2(input: &str) -> anyhow::Result<i64> {
    let mut inp = input;
    let data = full_input!(parse_sword_data_list)
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("Failed to parse input data: {} '{}'", e, inp))?;
    let qualities: Vec<i64> = data
        .iter()
        .map(|d| Fishbone::from_values(&d.values).quality())
        .collect();
    let min_value = qualities.iter().min().copied().unwrap_or_default();
    let max_value = qualities.iter().max().copied().unwrap_or_default();
    Ok(max_value - min_value)
}

fn part3(input: &str) -> anyhow::Result<i64> {
    let mut inp = input;
    let mut data = full_input!(parse_sword_data_list)
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("Failed to parse input data: {} '{}'", e, inp))?;
    data.sort_by(|a, b| compare_swords(b, a));
    let result = data
        .iter()
        .enumerate()
        .map(|(i, d)| (i, d.id))
        .inspect(|(i, v)| println!("Sword {}: {}", i + 1, v))
        .map(|(i, v)| ((i + 1) as i64) * v)
        .sum::<i64>();
    Ok(result)
}

#[derive(Debug, Clone)]
struct SwordData {
    id: i64,
    values: Vec<i64>,
}

type Item = (Option<i64>, i64, Option<i64>);
#[derive(Debug, Clone)]
struct Fishbone {
    values: Vec<Item>,
}

impl Fishbone {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn from_values(values: &[i64]) -> Self {
        let mut solution = Fishbone::new();
        for v in values {
            solution.add_value(*v);
        }
        solution
    }

    fn add_value(&mut self, value: i64) {
        for item in self.values.iter_mut() {
            match item {
                (None, v, right) if value < *v => {
                    *item = (Some(value), *v, *right);
                    return;
                }
                (left, v, None) if value > *v => {
                    *item = (*left, *v, Some(value));
                    return;
                }
                _ => {}
            }
        }
        self.values.push((None, value, None));
    }

    fn quality(&self) -> i64 {
        let mut s = String::new();
        for v in self.values.iter().map(|(_, v, _)| *v) {
            s.push_str(&v.to_string());
        }
        
        s.parse::<i64>().expect("cannot parse i64")
    }
}

fn compare_swords(s1: &SwordData, s2: &SwordData) -> Ordering {
    let fb1 = Fishbone::from_values(&s1.values);
    let fb2 = Fishbone::from_values(&s2.values);
    let q1 = fb1.quality();
    let q2 = fb2.quality();

    let c = q1.cmp(&q2);
    if c != Ordering::Equal {
        return c;
    }

    let l1 = fishbone_values(&fb1.values);
    let l2 = fishbone_values(&fb2.values);

    let c = l1.cmp(&l2);
    if c != Ordering::Equal {
        return c;
    }

    s1.id.cmp(&s2.id)
}

fn fishbone_values(items: &[Item]) -> Vec<i64> {
    items.iter().map(fishbone_value).collect()
}

fn fishbone_value(item: &Item) -> i64 {
    let (v1, v2, v3) = item;
    let mut s = String::new();
    if let Some(v) = v1 {
        s.push_str(&v.to_string());
    }
    s.push_str(&v2.to_string());
    if let Some(v) = v3 {
        s.push_str(&v.to_string());
    }
    
    s.parse::<i64>().expect("cannot parse")
}

fn parse_sword_data_list(input: &mut &str) -> ModalResult<Vec<SwordData>> {
    separated(1.., parse_sword_data, multispace1).parse_next(input)
}

fn parse_sword_data(input: &mut &str) -> ModalResult<SwordData> {
    (parse_id, parse_int_list)
        .map(|(id, values)| SwordData { id, values })
        .parse_next(input)
}

fn parse_id(input: &mut &str) -> ModalResult<i64> {
    terminated(parse_int, ':').parse_next(input)
}

fn parse_int_list(input: &mut &str) -> ModalResult<Vec<i64>> {
    separated(1.., parse_int, ',').parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<i64> {
    digit1.parse_to::<i64>().parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "58:5,3,7,8,9,10,4,5,7,8,8";
        let result = part1(input).expect("should work");
        assert_eq!(result, 581078);
    }

    #[test]
    fn test_part2() {
        let input = r#"1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5"#;
        let result = part2(input).expect("should work");
        assert_eq!(result, 77053);
    }

    #[test]
    fn test_part3() {
        let input = r#"1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7"#;
        let result = part3(input).expect("should work");
        assert_eq!(result, 260);
    }

    #[test]
    fn test_part3_2() {
        let input = r#"1:7,1,9,1,6,9,8,3,7,2
2:7,1,9,1,6,9,8,3,7,2"#;
        let result = part3(input).expect("should work");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_hase() {
        let input = "1:5,3,7,8,1,10,9,5,7,8";
        let mut inp = input;
        let sword = parse_sword_data.parse_next(&mut inp).expect("whatever");
        println!("sword: {sword:?}");
        let solution = Fishbone::from_values(&sword.values);
        println!("fishbone: {solution:?}");
        println!("quality: {}", solution.quality());
        let fb = fishbone_values(&solution.values);
        println!("fishbone: {fb:?}");
    }
}
