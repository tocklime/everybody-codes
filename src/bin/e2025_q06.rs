use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q06_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q06_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q06_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", part3::<1000,1000>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let mut counts : HashMap<char, usize> = HashMap::new();
    // let mut count = 0;
    let mut total = 0;
    for c in input.chars() {
        if c.is_ascii_uppercase() {
            *counts.entry(c).or_default() += 1;
        } else {
            if c == 'a' || PART > 1 {
                total += *counts.entry(c.to_ascii_uppercase()).or_default()
            }
        }
    }
    total
}
fn part3<const REPEATS: usize, const RANGE: usize>(input: &str) -> usize {
    //need to know total going left, and total going right.
    //how to drop the old knights?
    let chars = input.chars().collect_vec();

    // println!("\n\n {REPEATS} {RANGE}");
    //novice going left.
    let mut total = 0;
    let mut counts : HashMap<char,VecDeque<usize>> = HashMap::new();
    for rep in 0..REPEATS {
        let mut total_this_time = 0;
        //rep 0 is the edge case. rep 1 is when we can wrap.
        for (ix, c) in chars.iter().enumerate() {
            let pos = rep * chars.len() + ix;
            if c.is_ascii_uppercase() {
                counts.entry(*c).or_default().push_back(pos);
            } else {
                let mentors = counts.entry(c.to_ascii_uppercase()).or_default();
                while let Some(m) = mentors.front() {
                    if pos - m > RANGE {
                        mentors.pop_front();
                    } else {
                        break;
                    }
                }
                // println!("novice {c} at {pos} has {} possible mentors to the left: {mentors:?}", mentors.len());
                total_this_time += mentors.len();
            }
        }
        // println!("LEFT rep {rep} has {total_this_time}");
        if rep * chars.len() > RANGE {
            //RANGE was past the start at the beginning of this loop. All future iterations of this outer loop will calculate this value.
            // println!("LEFT Breaking with {total_this_time} on rep {rep}. Range {RANGE} chars {}", chars.len());
            total += total_this_time * (REPEATS-rep);
            break;
        } else {
            total += total_this_time
        }
    }
    //right.
    let mut counts : HashMap<char,VecDeque<usize>> = HashMap::new();

    for rep in (0..REPEATS).rev() {
        //rep 0 is the edge case. rep 1 is when we can wrap.
        let mut total_this_time = 0;
        for (ix, c) in chars.iter().enumerate().rev() {
            let pos = rep*chars.len() + ix;
            if c.is_ascii_uppercase() {
                counts.entry(*c).or_default().push_back(pos);
            } else {
                let mentors = counts.entry(c.to_ascii_uppercase()).or_default();
                while let Some(m) = mentors.front() {
                    if *m > pos && m - pos > RANGE {
                        mentors.pop_front();
                    } else {
                        break;
                    }
                }
                // println!("novice {c} at {pos} has {} possible mentors to the right: {mentors:?}", mentors.len());
                total_this_time += mentors.len();
            }
        }
        // println!("RIGHT rep {rep} has {total_this_time}");
        if (REPEATS-1-rep) * chars.len() > RANGE {
            //RANGE was past the start at the beginning of this loop. All future iterations of this outer loop will calculate this value.
            // println!("RIGHT Breaking with {total_this_time} on rep {rep}. Range {RANGE} chars {}", chars.len());
            total += total_this_time * (rep+1);
            break;
        } else {
            total += total_this_time
        }
    }
    total
}


#[cfg(test)]
mod test {
    use super::*;
    const EG1 : &str = "ABabACacBCbca";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 5);
    }
    #[test]
    fn p3_example() {
        assert_eq!(part3::<1,10>("AABCBABCABCabcabcABCCBAACBCa"),34);
        assert_eq!(part3::<2,10>("AABCBABCABCabcabcABCCBAACBCa"),72);
        assert_eq!(part3::<1000,1000>("AABCBABCABCabcabcABCCBAACBCa"),3442321);
    }
}