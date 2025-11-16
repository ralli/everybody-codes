use anyhow::anyhow;
use std::fmt;
use std::path::Path;
use winnow::ModalResult;
use winnow::Parser;
use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{delimited, eof, opt, separated_pair};

fn main() -> anyhow::Result<()> {
    let input = read_input("everybody_codes_e2025_q02_p1.txt")?;
    let result = part1(&input)?;
    println!("{result}");
    let input = read_input("everybody_codes_e2025_q02_p2.txt")?;
    let result = part2(&input)?;
    println!("{result}");
    let input = read_input("everybody_codes_e2025_q02_p3.txt")?;
    let result = part3(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<Complex> {
    let mut inp = input;
    let a = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    let ten = Complex { x: 10, y: 10 };
    let n = (0..3).fold(Complex { x: 0, y: 0 }, |r, _| {
        let x = r.mul(&r);
        let x = x.div(&ten);
        let x = x.add(&a);
        x
    });
    Ok(n)
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let a = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    let mut count = 0;
    for y in 0..=100 {
        for x in 0..=100 {
            let z = Complex {
                x: a.x + x * 10,
                y: a.y + y * 10,
            };
            if should_plot(&z) {
                count += 1;
            }
        }
        // println!();
    }
    Ok(count)
}

fn part3(input: &str) -> anyhow::Result<usize> {
    let mut inp = input;
    let a = parse_input_data(&mut inp).map_err(|e| anyhow!("{e}"))?;
    let mut count = 0;
    for y in 0..=1000 {
        for x in 0..=1000 {
            let z = Complex {
                x: a.x + x,
                y: a.y + y,
            };
            if should_plot(&z) {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn should_plot(p: &Complex) -> bool {
    let divisor = Complex {
        x: 100000,
        y: 100000,
    };
    let mut r = Complex { x: 0, y: 0 };
    for _i in 1..=100 {
        r = r.mul(&r);
        r = r.div(&divisor);
        r = r.add(&p);
        if !(-1_000_000..=1_000_000).contains(&r.x) || !(-1_000_000..=1_000_000).contains(&r.y) {
            // println!("p={p} r={r} c={i}");
            return false;
        }
    }
    true
}

fn read_input(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let result = std::fs::read_to_string(path)?;
    Ok(result)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Complex {
    x: i64,
    y: i64,
}

impl Complex {
    fn add(&self, other: &Complex) -> Complex {
        Complex {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn mul(&self, other: &Complex) -> Complex {
        Complex {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }

    fn div(&self, other: &Complex) -> Complex {
        Complex {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

fn parse_input_data(input: &mut &str) -> ModalResult<Complex> {
    delimited("A=", parse_complex, (multispace0, eof)).parse_next(input)
}

fn parse_complex(input: &mut &str) -> ModalResult<Complex> {
    delimited('[', separated_pair(parse_int, ',', parse_int), ']')
        .map(|(x, y)| Complex { x, y })
        .parse_next(input)
}

fn parse_int(input: &mut &str) -> ModalResult<i64> {
    (opt('-'), digit1)
        .take()
        .parse_to::<i64>()
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut input = "A=[25,9]";
        let result = part1(input).expect("should work");
        assert_eq!(result, Complex { x: 357, y: 862 });
    }

    #[test]
    fn test_part2() {
        let mut input = "A=[35300,-64910]";
        let result = part2(input).expect("should work");
        assert_eq!(result, 4076);
    }

    #[test]
    fn test_part3() {
        let mut input = "A=[35300,-64910]";
        let result = part3(input).expect("should work");
        assert_eq!(result, 406954);
    }

    #[test]
    fn test_should_plot() {
        let p = Complex {
            x: 35460,
            y: -64910,
        };
        assert!(should_plot(&p) == false);
    }
}
