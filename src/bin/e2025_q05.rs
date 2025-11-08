use std::cmp::Reverse;

use everybody_codes::nums::digits;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    Parser,
};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q05_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q05_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q05_p3.txt");

fn main() {
    println!("P1: {}", assess_fishbone(P1_INPUT));
    println!("P2: {}", find_quality_range(P2_INPUT));
    println!("P3: {}", get_checksum(P3_INPUT));
}

fn concat_digits<T: IntoIterator<Item = u64>>(t: T) -> u64 {
    t.into_iter()
        .flat_map(digits)
        .fold(0, |acc, d| 10 * acc + d)
}

struct Segment {
    left: Option<u64>,
    spine: u64,
    right: Option<u64>,
}
impl Segment {
    fn new(spine: u64) -> Self {
        Self {
            spine,
            left: None,
            right: None,
        }
    }
    fn insert(&mut self, n: u64) -> bool {
        if n < self.spine && self.left.is_none() {
            self.left = Some(n);
            true
        } else if n > self.spine && self.right.is_none() {
            self.right = Some(n);
            true
        } else {
            false
        }
    }
    fn level_number(&self) -> u64 {
        concat_digits(
            [self.left, Some(self.spine), self.right]
                .iter()
                .filter_map(|x| *x),
        )
    }
}
struct Fishbone {
    id: u64,
    segments: Vec<Segment>,
}
impl Fishbone {
    fn parser<'a>() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        (
            complete::u64,
            tag(":"),
            nom::multi::separated_list1(tag(","), complete::u64),
        )
            .map(|(id, _, ns)| Self::new(id, &ns))
    }
    fn new(id: u64, ns: &[u64]) -> Self {
        let mut segments: Vec<Segment> = Vec::new();
        for &n in ns {
            if segments.iter_mut().position(|x| x.insert(n)).is_none() {
                segments.push(Segment::new(n));
            }
        }
        Self { id, segments }
    }
    fn make_cmp_key(&self) -> (u64, Vec<u64>, u64) {
        (
            self.quality(),
            self.segments.iter().map(|x| x.level_number()).collect(),
            self.id,
        )
    }
    fn quality(&self) -> u64 {
        concat_digits(self.segments.iter().map(|x| x.spine))
    }
}

fn parse(input: &str) -> Vec<Fishbone> {
    nom::multi::separated_list1(newline, Fishbone::parser())
        .parse(input)
        .unwrap()
        .1
}

fn assess_fishbone(input: &str) -> u64 {
    parse(input)[0].quality()
}
fn get_checksum(input: &str) -> u64 {
    let mut fishbones: Vec<Fishbone> = parse(input);
    fishbones.sort_by_cached_key(|x| Reverse(x.make_cmp_key()));
    fishbones
        .iter()
        .map(|x| x.id)
        .zip(1..)
        .map(|(id, ix)| id * ix)
        .sum()
}

fn find_quality_range(input: &str) -> u64 {
    let x = parse(input).iter().map(Fishbone::quality).minmax();
    match x {
        itertools::MinMaxResult::NoElements => panic!(),
        itertools::MinMaxResult::OneElement(_) => 0,
        itertools::MinMaxResult::MinMax(a, b) => a.abs_diff(b),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "58:5,3,7,8,9,10,4,5,7,8,8";
    #[test]
    fn p1_example() {
        assert_eq!(assess_fishbone(EG1), 581078);
    }
    #[test]
    fn p2_example() {
        const EG: &str = "1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5";
        assert_eq!(find_quality_range(EG), 77053);
    }
    #[test]
    fn p3_example() {
        const EG: &str = "1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7";
        assert_eq!(get_checksum(EG), 260);
    }
    #[test]
    fn correct_answers() {
        assert_eq!(assess_fishbone(P1_INPUT), 5682487436);
        assert_eq!(find_quality_range(P2_INPUT), 8829332495717);
        assert_eq!(get_checksum(P3_INPUT), 30970476);
    }
}
