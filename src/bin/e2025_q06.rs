use std::{collections::VecDeque, usize};

use everybody_codes::collections::VecLookup;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q06_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q06_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q06_p3.txt");

fn main() {
    println!("P1: {}", part1(P1_INPUT));
    println!("P2: {}", part2(P2_INPUT));
    println!("P3: {}", part3::<1000, 1000>(P3_INPUT));
}
fn part1(input: &str) -> usize {
    input
        .chars()
        .fold((0, 0), |(mentors, options), c| match c {
            'a' => (mentors, mentors + options),
            'A' => (mentors + 1, options),
            _ => (mentors, options),
        })
        .1
}
fn part2(input: &str) -> usize {
    count_novices::<1, { usize::MAX }>(input.as_bytes())
}

fn count_novices<const REPEATS: usize, const RANGE: usize>(chars: &[u8]) -> usize {
    let mut total = 0;
    //VecLookup<X> acts like a HashMap<usize,X>, but is backed by a Vec.
    //we use A=0,B=1 etc as the keys.
    let mut counts: VecLookup<VecDeque<usize>> = VecLookup::with_capacity(26);
    let mut pos = 0;
    for rep in 0..REPEATS {
        let mut total_this_time = 0;
        for c in chars {
            if c.is_ascii_uppercase() {
                counts.entry((c - b'A').into()).or_default().push_back(pos);
            } else {
                let mentors = counts
                    .entry((c.to_ascii_uppercase() - b'A').into())
                    .or_default();
                while let Some(m) = mentors.front() {
                    if pos - m > RANGE {
                        mentors.pop_front();
                    } else {
                        break;
                    }
                }
                total_this_time += mentors.len();
            }
            pos += 1;
        }
        if rep * chars.len() > RANGE {
            //pos-RANGE was past the start at the beginning of this loop. 
            //All future iterations of this outer loop will calculate this value.
            //so we short circuit the rest.
            total += total_this_time * (REPEATS - rep);
            break;
        } else {
            total += total_this_time
        }
    }
    total
}

fn part3<const REPEATS: usize, const RANGE: usize>(input: &str) -> usize {
    //need to know total going left, and total going right.
    let mut total = count_novices::<REPEATS, RANGE>(input.as_bytes());
    let backward = input.as_bytes().iter().rev().copied().collect::<Vec<u8>>();
    total += count_novices::<REPEATS, RANGE>(&backward);
    total
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "ABabACacBCbca";
    #[test]
    fn p1_example() {
        assert_eq!(part1(EG1), 5);
    }
    #[test]
    fn p3_example() {
        assert_eq!(part3::<1, 10>("AABCBABCABCabcabcABCCBAACBCa"), 34);
        assert_eq!(part3::<2, 10>("AABCBABCABCabcabcABCCBAACBCa"), 72);
        assert_eq!(part3::<1000, 1000>("AABCBABCABCabcabcABCCBAACBCa"), 3442321);
    }

    #[test]
    fn correct_answers() {
        assert_eq!(part1(P1_INPUT), 154);
        assert_eq!(part2(P2_INPUT), 3700);
        assert_eq!(part3::<1000, 1000>(P3_INPUT), 1664019012);
    }
}
