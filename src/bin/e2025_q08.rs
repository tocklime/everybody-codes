use std::collections::HashMap;

use everybody_codes::collections::VecLookup;
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q08_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q08_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q08_p3.txt");

fn main() {
    println!("P1: {}", solve::<1, 32>(P1_INPUT));
    println!("P2: {}", solve::<2,256>(P2_INPUT));
    println!("P3: {}", solve::<3,256>(P3_INPUT));
}
fn sort(a: usize, b: usize) -> (usize,usize) {
    (a.min(b), a.max(b))
}
fn solve<const PART: usize, const NAILS: usize>(input: &str) -> usize {
    let ns: Vec<usize> = input.split(",").map(|x| x.parse().unwrap()).collect();
    match PART {
        1 => {
            let target_diff = NAILS / 2;
            ns.iter()
                .tuple_windows()
                .filter(|(a, b)| a.abs_diff(**b) == target_diff)
                .count()
        }
        2 => {
            let mut done_pairs : HashMap<usize, VecLookup<usize>> = HashMap::new();
            let mut total_crossings = 0;
            for (&a,&b) in ns.iter().tuple_windows() {
                let (sm,bi) = sort(a,b);
                let r = sm+1..bi;
                let mut crossed_now = 0;
                for x in r {
                    let x = done_pairs.entry(x).or_default();
                    for y in (1..sm).chain(bi+1..=NAILS) {
                        crossed_now += *x.entry(y).or_default();
                    }
                }
                *done_pairs.entry(sm).or_default().entry(bi).or_default() += 1;
                *done_pairs.entry(bi).or_default().entry(sm).or_default() += 1;
                // *done_pairs.entry((sm,bi)).or_default() += 1;
                // println!("{a}->{b} crossed {crossed_now} things");
                total_crossings += crossed_now;
            }
            total_crossings
        }
        3 => {
            let mut done_pairs : HashMap<usize, VecLookup<usize>> = HashMap::new();
            let mut max_crossings = 0;
            for (&a,&b) in ns.iter().tuple_windows() {
                *done_pairs.entry(a).or_default().entry(b).or_default() += 1;
                *done_pairs.entry(b).or_default().entry(a).or_default() += 1;
            }

            for sm in 1..=NAILS {
                for bi in sm..=NAILS {
                    let r = sm+1..bi;
                    let mut crossed_now = *done_pairs.entry(sm).or_default().entry(bi).or_default();
                    for x in r {
                        let x = done_pairs.entry(x).or_default();
                        for y in (1..sm).chain(bi+1..=NAILS) {
                            crossed_now += *x.entry(y).or_default();
                        }
                    }
                    // *done_pairs.entry((sm,bi)).or_default() += 1;
                    max_crossings = max_crossings.max(crossed_now);
                }
            }
            max_crossings
        }
        _ => unimplemented!(),
    }
}
//1497013 is right len wrong first digit.
//2792 right len right first digit.
//2791 right len right first digit.
#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "1,5,2,6,8,4,1,7,3";
    const EG2: &str = "1,5,2,6,8,4,1,7,3,5,7,8,2";
    const EG3: &str = "1,5,2,6,8,4,1,7,3,6";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1, 8>(EG1), 4);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2,8>(EG2), 21);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3,8>(EG3), 7);
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1, 32>(P1_INPUT), 63);
        assert_eq!(solve::<2, 256>(P2_INPUT), 2925244);
        assert_eq!(solve::<3, 256>(P3_INPUT), 2795);
    }
}
