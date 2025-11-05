use std::ops::{Add, Div, Mul};

use nom::{bytes::complete::tag, character::complete::i64, Parser};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q02_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q02_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q02_p3.txt");

#[derive(Debug, Copy, Clone, Default)]
struct ComplexNum {
    x: i64,
    y: i64,
}
impl From<(i64, i64)> for ComplexNum {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl std::fmt::Display for ComplexNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{},{}]", self.x, self.y))
    }
}

impl ComplexNum {
    fn parse<'a>() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        (tag("["), i64, tag(","), i64, tag("]")).map(|(_, x, _, y, _)| Self { x, y })
    }
    fn parse_line<'a>(
    ) -> impl Parser<&'a str, Output = (String, Self), Error = nom::error::Error<&'a str>> {
        (nom::character::complete::alpha1, tag("="), Self::parse())
            .map(|(c, _, cn)| (c.to_string(), cn))
    }
    fn process(self, reps: usize, divide_by: Self) -> Option<Self> {
        let mut total = Self::default();
        for _ in 0..reps {
            if total.is_big() {
                return None;
            }
            total = total * total;
            total = total / divide_by;
            total = total + self;
        }
        Some(total)
    }
    fn is_big(self) -> bool {
        self.x.abs() > 1000000 || self.y.abs() > 1000000
    }
    fn should_engrave(self) -> bool {
        let n = self.process(100, (100000, 100000).into());
        n.map(|x| !x.is_big()).unwrap_or_default()
    }
}

impl Add for ComplexNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Mul for ComplexNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x - (self.y * rhs.y),
            y: self.x * rhs.y + (self.y * rhs.x),
        }
    }
}
impl Div for ComplexNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> String {
    let (_, (_, a)) = ComplexNum::parse_line().parse(input).unwrap();
    match PART {
        1 => a.process(3, (10, 10).into()).unwrap().to_string(),
        2 => {
            let mut eng_count = 0;
            for x in (a.x..).step_by(10).take(101) {
                for y in (a.y..).step_by(10).take(101) {
                    let n = ComplexNum { x, y };
                    if n.should_engrave() {
                        eng_count += 1;
                    }
                }
            }
            format!("{eng_count}")
        }
        3 => {
            let mut eng_count = 0;
            for x in (a.x..).take(1001) {
                for y in (a.y..).take(1001) {
                    let n = ComplexNum { x, y };
                    if n.should_engrave() {
                        eng_count += 1;
                    }
                }
            }
            format!("{eng_count}")
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "A=[25,9]";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), "[357,862]");
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>("A=[35300,-64910]"), "4076");
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>("A=[35300,-64910]"), "406954");
    }
}
