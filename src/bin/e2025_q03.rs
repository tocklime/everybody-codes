use itertools::Itertools;
use std::collections::HashMap;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q03_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q03_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q03_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> u32 {
    let ns = input
        .split(",")
        .map(|x| str::parse(x).unwrap())
        .collect::<Vec<u32>>();
    match PART {
        1 => ns.into_iter().unique().sum(),
        2 => ns.into_iter().unique().sorted().take(20).sum(),
        3 => {
            let mut counts = HashMap::new();
            ns.into_iter().for_each(|n| {
                *counts.entry(n).or_default() += 1;
            });
            *counts.values().max().unwrap()
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "10,5,1,10,3,8,5,2,2";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 29);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>("4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77"), 781);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>("4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77"), 3);
    }
    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 2700);
        assert_eq!(solve::<2>(P2_INPUT), 213);
        assert_eq!(solve::<3>(P3_INPUT), 4541);
    }
}
