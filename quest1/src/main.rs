use anyhow::anyhow;
use std::path::Path;
use winnow::{
    ModalResult, Parser,
    ascii::{alpha1, digit1, line_ending},
    combinator::{alt, repeat, separated},
};

fn main() -> anyhow::Result<()> {
    let input = read_input("everybody_codes_e2025_q01_p1.txt")?;
    let result = part1(&input)?;
    println!("{result}");
    let input = read_input("everybody_codes_e2025_q01_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn read_input(filename: impl AsRef<Path>) -> anyhow::Result<String> {
    let contents = std::fs::read_to_string(filename)?;
    Ok(contents)
}

#[derive(Debug, Clone)]
struct InputData {
    names: Vec<String>,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    steps: i32,
}

// Ryththyris
// fn part1(input: &str) -> anyhow::Result<String> {
//     let mut inp = input;
//     let input_data = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
//     let mut idx = 0;
//     let size = input_data.names.len() as i32;
//     for instruction in input_data.instructions.iter() {
//         match instruction.direction {
//             Direction::Left => idx -= instruction.steps ,
//             Direction::Right => idx += instruction.steps,
//         }
//         if idx < 0 {
//             idx = 0;
//         }
//         if idx >= size {
//             idx = size - 1;
//         }
//     }
//     Ok(input_data.names[idx as usize].clone())
// }

fn part1(input: &str) -> anyhow::Result<String> {
    let mut inp = input;
    let input_data = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    let size = input_data.names.len() as i32;
    let idx =
        input_data
            .instructions
            .iter()
            .fold(0, |idx, instruction| match instruction.direction {
                Direction::Left => clamp(idx - instruction.steps, 0, size - 1),
                Direction::Right => clamp(idx + instruction.steps, 0, size - 1),
            }) as usize;
    Ok(input_data.names[idx].clone())
}

fn part2(input: &str) -> anyhow::Result<String> {
    let mut inp = input;
    let input_data = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    let size = input_data.names.len() as i32;
    let idx = input_data
        .instructions
        .iter()
        .fold(0i32, |idx, instruction| match instruction.direction {
            Direction::Left => (idx - instruction.steps).rem_euclid(size),
            Direction::Right => (idx + instruction.steps).rem_euclid(size),
        }) as usize;
    Ok(input_data.names[idx].clone())
}

fn clamp(n: i32, min: i32, max: i32) -> i32 {
    if n < min {
        return min;
    }
    if n > max {
        return max;
    }
    n
}
fn parse_input_data(input: &mut &str) -> ModalResult<InputData> {
    let names: Vec<String> =
        separated(1.., alpha1.map(|s: &str| s.to_string()), ",").parse_next(input)?;
    (line_ending, line_ending).parse_next(input)?;
    let instructions: Vec<Instruction> =
        separated(1.., parse_instruction, ",").parse_next(input)?;
    Ok(InputData {
        names,
        instructions,
    })
}

fn parse_instruction(input: &mut &str) -> ModalResult<Instruction> {
    (parse_direction, parse_int)
        .map(|(direction, steps)| Instruction { direction, steps })
        .parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<i32> {
    digit1.parse_to::<i32>().parse_next(input)
}

fn parse_direction(input: &mut &str) -> ModalResult<Direction> {
    alt(('L'.value(Direction::Left), 'R'.value(Direction::Right))).parse_next(input)
}

const INPUT: &str = r#"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"#;

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1"#;

    #[test]
    fn part1_works() {
        assert_eq!(part1(INPUT).expect("no error"), "Fyrryn");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(INPUT).expect("no error"), "Elarzris");
    }
}
