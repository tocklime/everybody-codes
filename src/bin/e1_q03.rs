use everybody_codes::{inputs::get_matches_from_str, nums::chinese_remainder_theorem};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e1_q03_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e1_q03_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e1_q03_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn sim_100(snails: &[Snail]) -> i64 {
    snails
        .iter()
        .map(|s| {
            //where after 100? well, x will be
            let (x, y) = s.position_after(100);
            x + (100 * y)
        })
        .sum()
}
#[derive(Debug)]
struct Snail {
    position: i64,
    period_len: i64,
}
impl Snail {
    fn position_after(&self, n: i64) -> (i64, i64) {
        let final_x = (self.position + n) % self.period_len;
        let final_y = self.period_len - self.position - 1;
        (final_x + 1, final_y + 1)
    }
}
fn parse_snails(input: &str) -> Vec<Snail> {
    input
        .lines()
        .map(|l| {
            let ns = get_matches_from_str(l, nom::character::complete::i64);
            assert_eq!(ns.len(), 2);
            let x = ns[0] - 1;
            let y = ns[1] - 1;
            let period_len = x + y + 1;
            Snail {
                position: x,
                period_len,
            }
        })
        .collect()
}
fn find_alignment(snails: &[Snail]) -> i64 {
    let conv = snails.iter().map(|s| (s.period_len - s.position - 1, s.period_len)).collect::<Vec<_>>();
    chinese_remainder_theorem(&conv)
}
fn solve<const PART: usize>(input: &str) -> i64 {
    let snails = parse_snails(input);
    match PART {
        1 => sim_100(&snails),
        2 | 3 => find_alignment(&snails),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "x=1 y=2
x=2 y=3
x=3 y=4
x=4 y=4";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 1310);
    }
    #[test]
    fn p2_example_a() {
        const EG2A: &str = "x=12 y=2
x=8 y=4
x=7 y=1
x=1 y=5
x=1 y=3";
        assert_eq!(solve::<2>(EG2A), 14);
    }
    #[test]
    fn p2_example_b() {
        const EG2B: &str = "x=3 y=1
x=3 y=9
x=1 y=5
x=4 y=10
x=5 y=3";
        assert_eq!(solve::<2>(EG2B), 13659);
    }
}
