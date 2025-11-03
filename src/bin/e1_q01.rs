use std::collections::HashMap;

use everybody_codes::{
    inputs::get_matches_from_str,
    nums::{digits, mod_pow},
};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e1_q01_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e1_q01_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e1_q01_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}

fn eni(n: u64, exp: u64, modulus: u64) -> u64 {
    let mut ans = Vec::new();
    let mut score = 1;
    for _ in 0..exp {
        score *= n;
        score %= modulus;
        ans.push(score);
    }
    ans.into_iter()
        .rev()
        .flat_map(digits)
        .fold(0, |acc, n| acc * 10 + n)
}

fn eni5(n: u64, exp: u64, modulus: u64) -> u64 {
    let mut ans = Vec::new();
    let fast_count = exp.saturating_sub(5);
    let slow_count = exp.min(5);
    let mut score = mod_pow(n, fast_count, modulus);
    for _ in 0..slow_count {
        score *= n;
        score %= modulus;
        ans.push(score);
    }
    ans.into_iter()
        .rev()
        .flat_map(digits)
        .fold(0, |acc, n| acc * 10 + n)
}
fn eni_sum_naive(mut score: u64, base: u64, exp: u64, modulus: u64) -> u64 {
    let mut ans = 0;
    for _ in 0..exp {
        score = score * base % modulus;
        ans += score;
    }
    ans
}
fn eni_sum(base: u64, exp: u64, modulus: u64) -> u64 {
    //this is cycle detection.
    let mut ans = 0;
    let mut score = 1;
    let mut seen_scores = HashMap::new();
    for ix in 0..exp {
        let next = score * base % modulus;
        if let Some((run_in, old_ans)) = seen_scores.insert(next, (ix, ans)) {
            let cycle_len = ix - run_in;
            let loop_count = (exp - run_in) / cycle_len;
            let run_out = (exp - run_in) % cycle_len;
            let loop_size = ans - old_ans;
            let run_out_sum = eni_sum_naive(score, base, run_out, modulus);
            return old_ans + loop_size * loop_count + run_out_sum;
        }
        score = next;
        ans += score;
    }
    ans
}

fn solve<const PART: usize>(input: &str) -> u64 {
    let func = match PART {
        1 => eni,
        2 => eni5,
        3 => eni_sum,
        _ => unimplemented!()
    };
    let best = input
        .lines()
        .map(|l| {
            let ns = get_matches_from_str(l, nom::character::complete::u64);
            if let &[a, b, c, x, y, z, m] = ns.as_slice() {
                func(a, x, m) + func(b, y, m) + func(c, z, m)
            } else {
                0
            }
        })
        .max()
        .unwrap();
    best
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "A=4 B=4 C=6 X=3 Y=4 Z=5 M=11
A=8 B=4 C=7 X=8 Y=4 Z=6 M=12
A=2 B=8 C=6 X=2 Y=4 Z=5 M=13
A=5 B=9 C=6 X=8 Y=6 Z=8 M=14
A=5 B=9 C=7 X=6 Y=6 Z=8 M=15
A=8 B=8 C=8 X=6 Y=9 Z=6 M=16";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 11611972920);
    }
    const EG2A: &str = "A=4 B=4 C=6 X=3 Y=14 Z=15 M=11
A=8 B=4 C=7 X=8 Y=14 Z=16 M=12
A=2 B=8 C=6 X=2 Y=14 Z=15 M=13
A=5 B=9 C=6 X=8 Y=16 Z=18 M=14
A=5 B=9 C=7 X=6 Y=16 Z=18 M=15
A=8 B=8 C=8 X=6 Y=19 Z=16 M=16";
    const EG2B: &str = "A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145";

    #[test]
    fn p2_examples() {
        assert_eq!(solve::<2>(EG2A), 11051340);
        assert_eq!(solve::<2>(EG2B), 1507702060886);
    }

    const EG3A: &str = "A=4 B=4 C=6 X=3000 Y=14000 Z=15000 M=110
A=8 B=4 C=7 X=8000 Y=14000 Z=16000 M=120
A=2 B=8 C=6 X=2000 Y=14000 Z=15000 M=130
A=5 B=9 C=6 X=8000 Y=16000 Z=18000 M=140
A=5 B=9 C=7 X=6000 Y=16000 Z=18000 M=150
A=8 B=8 C=8 X=6000 Y=19000 Z=16000 M=160";
    const EG3B: &str = "A=3657 B=3583 C=9716 X=903056852 Y=9283895500 Z=85920867478 M=188
A=6061 B=4425 C=5082 X=731145782 Y=1550090416 Z=87586428967 M=107
A=7818 B=5395 C=9975 X=122388873 Y=4093041057 Z=58606045432 M=102
A=7681 B=9603 C=5681 X=716116871 Y=6421884967 Z=66298999264 M=196
A=7334 B=9016 C=8524 X=297284338 Y=1565962337 Z=86750102612 M=145";

    #[test]
    fn p3_examples() {
        assert_eq!(eni_sum(2, 7, 5), 19);
        assert_eq!(eni_sum(3, 8, 16), 48);
        assert_eq!(solve::<3>(EG3A), 3279640);
        assert_eq!(solve::<3>(EG3B), 7276515438396);
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 8968620468);
        assert_eq!(solve::<2>(P2_INPUT), 125446570652405);
        assert_eq!(solve::<3>(P3_INPUT), 670903743627636);
    }
}
