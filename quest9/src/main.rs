use itertools::izip;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::read_to_string;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, line_ending, multispace0};
use winnow::combinator::{eof, separated_pair, terminated};
use winnow::combinator::{repeat, separated};
use winnow::token::one_of;

fn main() -> anyhow::Result<()> {
    let input = read_to_string("everybody_codes_e2025_q09_p1.txt")?;
    let result = part1(&input)?;
    println!("{result}");

    let input = read_to_string("everybody_codes_e2025_q09_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");

    let input = read_to_string("everybody_codes_e2025_q09_p3.txt")?;
    let result = part3(&input)?;
    println!("{result}");

    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let input_data = terminated(parse_sequences, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("Failed to parse sequence: {}", e))?;
    let degree1 = similarity_degree(&input_data[0], &input_data[2]);
    let degree2 = similarity_degree(&input_data[1], &input_data[2]);
    Ok(degree1 * degree2)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut result = 0;
    let mut inp = input;
    let input_data = terminated(parse_sequences, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("Failed to parse sequence: {}", e))?;
    for (i, a) in input_data.iter().enumerate() {
        for (j, b) in input_data.iter().enumerate().skip(i + 1) {
            for (_k, c) in input_data
                .iter()
                .enumerate()
                .filter(|(k, _c)| *k != i && *k != j)
            {
                if izip!(a.symbols.iter(), b.symbols.iter(), c.symbols.iter())
                    .all(|(ac, bc, cc)| ac == cc || bc == cc)
                {
                    let degree1 = similarity_degree(a, c);
                    let degree2 = similarity_degree(b, c);
                    result += degree1 * degree2;
                }
            }
        }
    }
    Ok(result)
}

fn part3(input: &str) -> anyhow::Result<i32> {
    let mut inp = input;
    let input_data = terminated(parse_sequences, (multispace0, eof))
        .parse_next(&mut inp)
        .map_err(|e| anyhow::anyhow!("Failed to parse sequence: {}", e))?;
    let mut adj: BTreeMap<i32, Vec<i32>> = BTreeMap::new();

    for (i, a) in input_data.iter().enumerate() {
        for (j, b) in input_data.iter().enumerate().skip(i + 1) {
            for (_k, c) in input_data
                .iter()
                .enumerate()
                .filter(|(k, _c)| *k != i && *k != j)
            {
                if izip!(a.symbols.iter(), b.symbols.iter(), c.symbols.iter())
                    .all(|(ac, bc, cc)| ac == cc || bc == cc)
                {
                    adj.entry(a.id).or_default().push(c.id);
                    adj.entry(b.id).or_default().push(c.id);
                    adj.entry(c.id).or_default().push(a.id);
                    adj.entry(c.id).or_default().push(b.id);
                }
            }
        }
    }

    let mut visited = BTreeSet::<i32>::new();
    let empty: Vec<i32> = Vec::new();
    let mut families: Vec<Vec<i32>> = Vec::new();
    for seq in input_data.iter() {
        if visited.contains(&seq.id) {
            continue;
        }
        let mut q = VecDeque::from([seq.id]);
        let mut curr: Vec<i32> = Vec::new();
        while let Some(i) = q.pop_front() {
            if !visited.insert(i) {
                continue;
            }
            curr.push(i);
            let neighbors = adj.get(&i).unwrap_or(&empty);
            for j in neighbors.iter().filter(|&j| !visited.contains(j)) {
                q.push_back(*j);
            }
        }
        families.push(curr);
    }
    let best = families.iter().max_by_key(|f| f.len()).unwrap_or(&empty);
    let result = best.iter().sum();
    Ok(result)
}

fn similarity_degree(seq1: &Sequence, seq2: &Sequence) -> usize {
    seq1.symbols
        .iter()
        .zip(seq2.symbols.iter())
        .filter(|(a, b)| a == b)
        .count()
}

#[derive(Debug, Clone)]
struct Sequence {
    id: i32,
    symbols: Vec<char>,
}

fn parse_sequences(input: &mut &str) -> ModalResult<Vec<Sequence>> {
    separated(1.., parse_sequence, line_ending).parse_next(input)
}

fn parse_sequence(input: &mut &str) -> ModalResult<Sequence> {
    separated_pair(parse_int, ':', repeat(1.., one_of(['A', 'T', 'C', 'G'])))
        .map(|(id, symbols)| Sequence { id, symbols })
        .parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<i32> {
    digit1.parse_to::<i32>().parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG
2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT
3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG"#;
        let result = part1(input).unwrap();
        assert_eq!(result, 414);
    }

    #[test]
    fn test_part2() {
        let input = r#"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG"#;
        let result = part2(input).unwrap();
        assert_eq!(result, 1245);
    }

    #[test]
    fn test_part3() {
        let input = r#"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG"#;
        let result = part3(input).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part3_2() {
        let input = r#"1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC
2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC
3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG
4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA
5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA
6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA
7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG
8:GGCGTAAAGTATGGATGCTGGCTAGGCACCCG"#;
        let result = part3(input).unwrap();
        assert_eq!(result, 36);
    }
}
