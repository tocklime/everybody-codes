use itertools::Itertools;
use num::rational::Ratio;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q04_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q04_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q04_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
//represent gears as tuple of (input teeth, output teeth).
fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|l| match l.split_once("|") {
            None => {
                let n: i64 = str::parse(l).unwrap();
                (n, n)
            }
            Some((a, b)) => (str::parse(a).unwrap(), str::parse(b).unwrap()),
        })
        .collect()
}

fn solve<const PART: usize>(input: &str) -> i64 {
    let gears = parse(input);
    let (n, backward) = match PART {
        1 => (2025, false),
        2 => (10000000000000, true),
        3 => (100, false),
        _ => unreachable!(),
    };
    let iter: Box<dyn Iterator<Item = (i64, i64)>> = if backward {
        //we swap the input/output numbers here. This doesn't actually affect the question as asked,
        //because only part 2 does the calculation backward, and we don't have stacked gears there.
        //A hypothetical part 4 might ask to do the calculation backward with the stacked gears.
        Box::new(gears.into_iter().map(|(a, b)| (b, a)).rev())
    } else {
        Box::new(gears.into_iter())
    };

    let count: Ratio<i64> = iter
        .tuple_windows()
        .fold(Ratio::<i64>::new(n, 1), |r, (a, b)| {
            r * Ratio::<i64>::new(a.1, b.0)
        });
    if backward {
        count.ceil()
    } else {
        count.floor()
    }
    .to_integer()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>("128\n64\n32\n16\n8"), 32400);
        assert_eq!(solve::<1>("102\n75\n50\n35\n13"), 15888);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>("128\n64\n32\n16\n8"), 625000000000);
        assert_eq!(solve::<2>("102\n75\n50\n35\n13"), 1274509803922);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>("5\n5|10\n10|20\n5"), 400);
        assert_eq!(solve::<3>("5\n7|21\n18|36\n27|27\n10|50\n10|50\n11"), 6818);
    }
    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 16463);
        assert_eq!(solve::<2>(P2_INPUT), 3471659919029);
        assert_eq!(solve::<3>(P3_INPUT), 329051481854);
    }
}
