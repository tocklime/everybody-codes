use everybody_codes::{collections::VecLookup, numset_bytes::NumSet};
use itertools::Itertools;
use nom::{
    branch::alt, bytes::tag, character::complete::i64, combinator::eof, multi::separated_list1,
    Parser,
};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q18_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q18_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q18_p3.txt");

#[derive(Debug)]
struct Branch {
    thickness: i64,
    connected: Option<i64>,
}

#[derive(Debug)]
struct Plant {
    id: i64,
    thickness: i64,
    connections: Vec<Branch>,
}

impl Branch {
    fn parse<'a>(
    ) -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        alt((
            (tag("- free branch with thickness "), i64).map(move |(_, i)| Self {
                thickness: i,
                connected: None,
            }),
            (tag("- branch to Plant "), i64, tag(" with thickness "), i64).map(
                move |(_, a, _, b)| Self {
                    thickness: b,
                    connected: Some(a),
                },
            ),
        ))
    }
}
impl Plant {
    fn parse<'a>() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        (tag("Plant "), i64, tag(" with thickness "), i64, tag(":\n")).flat_map(
            |(_, id, _, thickness, _)| {
                separated_list1(tag("\n"), Branch::parse()).map(move |connections| Self {
                    id,
                    thickness,
                    connections,
                })
            },
        )
    }
}
#[derive(Debug)]
struct TestCase {
    cases: Vec<i64>,
}
impl TestCase {
    fn parse<'a>() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        separated_list1(tag(" "), i64).map(|cases| Self { cases })
    }
}

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}

fn calc_bright(plants: &[Plant], id: i64, known: &mut VecLookup<i64>) -> i64 {
    if let Some(b) = known.get(id as usize) {
        return *b;
    }
    let me = &plants[(id - 1) as usize];
    let mut b = me
        .connections
        .iter()
        .map(|c| {
            if let Some(other_p) = c.connected {
                c.thickness * calc_bright(plants, other_p, known)
            } else {
                c.thickness * 1
            }
        })
        .sum();
    if b < me.thickness {
        b = 0;
    }
    // println!("Brightness of {id} is {b}");
    known.insert(id as usize, b);
    b
}
fn find_max(plants: &[Plant]) -> i64 {
    let mut to_test: VecLookup<NumSet<1>> = Default::default();
    let mut base_plants_to = 0;
    for p in plants {
        if p.connections.len() == 1 {
            base_plants_to = p.id;
        } else {
            for c in &p.connections {
                if let Some(prev) = c.connected {
                    if prev <= base_plants_to {
                        to_test
                            .entry(prev as usize)
                            .or_default().insert(if c.thickness > 0 { 1usize } else { 0 });
                    }
                }
            }
        }
    }
    let max = to_test.values().map(|x| x.iter()).multi_cartesian_product().map(|x| {
        let xv : Vec<i64> = x.iter().map(|x| *x as i64).collect();
        run_test(&plants, &xv)
    }).max().unwrap();
    max
}
fn run_test(plants: &[Plant], test: &[i64]) -> i64 {
    let mut known: VecLookup<i64> = Default::default();
    for n in test.iter().zip(1..) {
        known.insert(n.1, *n.0);
    }
    let last_plant = plants.last().unwrap().id;
    calc_bright(&plants, last_plant, &mut known)
}
fn solve<const PART: usize>(input: &str) -> i64 {
    let (input, plants) = separated_list1(tag("\n\n"), Plant::parse())
        .parse_complete(input)
        .unwrap();
    let (_, tests) = alt((
        (
            tag("\n\n\n"),
            nom::multi::separated_list0(tag("\n"), TestCase::parse()),
        )
            .map(|x| x.1),
        (eof).map(|_| Vec::new()),
    ))
    .parse_complete(input)
    .unwrap();
    let last_plant = plants.len() as i64;
    match PART {
        1 => {
            let mut known: VecLookup<i64> = Default::default();
            calc_bright(&plants, last_plant, &mut known)
        }
        2 => tests
            .iter()
            .map(|t| {
                let mut known: VecLookup<i64> = Default::default();
                for n in t.cases.iter().zip(1..) {
                    known.insert(n.1, *n.0);
                }
                calc_bright(&plants, last_plant, &mut known)
            })
            .sum(),
        3 => {
            let max = find_max(&plants);
            tests.iter().map(|t| {
                let v = run_test(&plants, &t.cases);
                if v > 0 {
                    max - v
                } else {
                    0
                }
            }).sum()
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 17:
- branch to Plant 1 with thickness 15
- branch to Plant 2 with thickness 3

Plant 5 with thickness 24:
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13

Plant 6 with thickness 15:
- branch to Plant 3 with thickness 14

Plant 7 with thickness 10:
- branch to Plant 4 with thickness 15
- branch to Plant 5 with thickness 21
- branch to Plant 6 with thickness 34";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 774);
    }
    #[test]
    fn p2_example() {
        assert_eq!(
            solve::<2>(
                "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 10:
- branch to Plant 1 with thickness -25
- branch to Plant 2 with thickness 17
- branch to Plant 3 with thickness 12

Plant 5 with thickness 14:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -26
- branch to Plant 3 with thickness 15

Plant 6 with thickness 150:
- branch to Plant 4 with thickness 5
- branch to Plant 5 with thickness 6


1 0 1
0 0 1
0 1 1"
            ),
            324
        );
    }
    #[test]
    fn p3_example() {
        assert_eq!(
            solve::<3>(
                "Plant 1 with thickness 1:
- free branch with thickness 1

Plant 2 with thickness 1:
- free branch with thickness 1

Plant 3 with thickness 1:
- free branch with thickness 1

Plant 4 with thickness 1:
- free branch with thickness 1

Plant 5 with thickness 8:
- branch to Plant 1 with thickness -8
- branch to Plant 2 with thickness 11
- branch to Plant 3 with thickness 13
- branch to Plant 4 with thickness -7

Plant 6 with thickness 7:
- branch to Plant 1 with thickness 14
- branch to Plant 2 with thickness -9
- branch to Plant 3 with thickness 12
- branch to Plant 4 with thickness 9

Plant 7 with thickness 23:
- branch to Plant 5 with thickness 17
- branch to Plant 6 with thickness 18


0 1 0 0
0 1 0 1
0 1 1 1
1 1 0 1"
            ),
            946
        );
    }

    // #[test]
    // fn correct_answers() {
    //     assert_eq!(solve::<1>(P1_INPUT), 0);
    //     assert_eq!(solve::<2>(P2_INPUT), 0);
    //     assert_eq!(solve::<3>(P3_INPUT), 0);
    // }
}
