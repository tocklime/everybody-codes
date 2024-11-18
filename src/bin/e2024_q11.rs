use std::collections::HashMap;

use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q11_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q11_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q11_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let map: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|l| {
            let (key, res) = l.split_once(':').unwrap();
            let res: Vec<&str> = res.split(',').collect();
            (key, res)
        })
        .collect();

    if PART < 3 {
        let (days, start) = match PART {
            1 => (4, "A"),
            2 => (10, "Z"),
            _ => unimplemented!(),
        };
        let mut termites = vec![start];
        for _d in 0..days {
            termites = termites
                .into_iter()
                .flat_map(|x| &map[&x])
                .cloned()
                .collect();
        }
        termites.len()
    } else {
        let mut memo = HashMap::new();
        let pops: Vec<usize> = map
            .keys()
            .map(|_k| recur(&mut memo, &map, _k, 20))
            .sorted()
            .collect();
        pops[pops.len() - 1] - pops[0]
    }
}
fn recur<'a>(
    memo: &mut HashMap<(&'a str, usize), usize>,
    map: &HashMap<&'a str, Vec<&'a str>>,
    termite: &'a str,
    day_count: usize,
) -> usize {
    if let Some(x) = memo.get(&(termite, day_count)) {
        return *x;
    }
    if day_count == 0 {
        return 1;
    }
    let mut ans = 0;
    for &x in &map[&termite] {
        ans += recur(memo, map, x, day_count - 1)
    }
    memo.insert((termite, day_count), ans);
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "A:B,C
B:C,A
C:A";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 8);
    }
    const EG3: &str = "A:B,C
B:C,A,A
C:A";
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG3), 268815);
    }
}
