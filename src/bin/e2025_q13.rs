use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q13_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q13_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q13_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> i64 {
    let ns: Vec<(i64, i64)> = input
        .lines()
        .map(|l| {
            if let Some((a, b)) = l.split_once('-') {
                (a.parse().unwrap(), b.parse().unwrap())
            } else {
                let a = l.parse().unwrap();
                (a, a)
            }
        })
        .collect();
    let total = 1 + ns.iter().map(|(a, b)| 1 + b - a).sum::<i64>();
    let count = match PART {
        1 => 2025,
        2 => 20252025,
        3 => 202520252025,
        _ => unreachable!(),
    };
    let ix = count % total;
    let mut f = Vec::new();
    f.push((1, 1, 1));
    //add all the even indexes.
    for ix in (0..ns.len()).step_by(2) {
        let (a, b) = ns[ix];
        f.push((a, 1, b - a + 1))
    }
    //add all the odd indexes, backward.
    for ix in (0..ns.len()).rev().step_by(2) {
        let (a, b) = ns[ix];
        f.push((b, -1, b - a + 1));
    }
    f.into_iter()
        .fold_while(ix, |rem, (start, delta, count)| {
            if rem >= count {
                Continue(rem - count)
            } else {
                Done(start + (rem * delta))
            }
        })
        .into_inner()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "72
58
47
61
67";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 67);
    }
    #[test]
    fn p2_example() {
        const EG2: &str = "10-15
12-13
20-21
19-23
30-37";
        assert_eq!(solve::<2>(EG2), 30);
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 212);
        assert_eq!(solve::<2>(P2_INPUT), 3500);
        assert_eq!(solve::<3>(P3_INPUT), 170354);
    }
}
