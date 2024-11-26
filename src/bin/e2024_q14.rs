use std::collections::HashSet;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q14_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q14_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q14_p3.txt");

type C = (i64, i64, i64);

fn main() {
    println!("P1: {}", solve1(P1_INPUT));
    println!("P2: {}", solve2(P2_INPUT));
    println!("P3: {}", solve3(P3_INPUT));
}
fn grow(input: &str) -> impl Iterator<Item = C> + use<'_> {
    let mut st = (0, 0, 0);
    input.split(',').flat_map(move |item| {
        let (instr, count) = item.split_at(1);
        let count: i64 = count.parse().unwrap();
        let mut this_step = Vec::with_capacity(count as usize);
        for _ in 0..count {
            match instr {
                "U" => st.1 += 1,
                "D" => st.1 -= 1,
                "L" => st.0 -= 1,
                "R" => st.0 += 1,
                "F" => st.2 -= 1,
                "B" => st.2 += 1,
                _ => unreachable!(),
            };
            this_step.push(st);
        }
        this_step.into_iter()
    })
}
fn solve1(input: &str) -> i64 {
    grow(input).map(|x| x.1).max().unwrap()
}
fn solve2(input: &str) -> usize {
    input.lines().flat_map(grow).collect::<HashSet<C>>().len()
}
fn neighbours(p: C, seen: &HashSet<C>) -> Vec<(C, u64)> {
    [
        ((p.0 - 1, p.1, p.2), 1),
        ((p.0 + 1, p.1, p.2), 1),
        ((p.0, p.1 - 1, p.2), 1),
        ((p.0, p.1 + 1, p.2), 1),
        ((p.0, p.1, p.2 - 1), 1),
        ((p.0, p.1, p.2 + 1), 1),
    ]
    .into_iter()
    .filter(|p| seen.contains(&p.0))
    .collect()
}
fn hamming(a: C, b: C) -> u64 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1) + a.2.abs_diff(b.2)
}
fn solve3(input: &str) -> u64 {
    let mut seen = HashSet::new();
    let mut leaves = HashSet::new();
    let mut max_height = 0;
    for x in input.lines() {
        let mut last = None;
        for x in grow(x) {
            seen.insert(x);
            last = Some(x);
            max_height = max_height.max(x.1);
        }
        leaves.insert(last.unwrap());
    }
    (0..=max_height)
        .map(|i| {
            let p = (0i64, i, 0i64);
            if seen.contains(&p) {
                leaves
                    .iter()
                    .map(|&l| {
                        pathfinding::directed::astar::astar(
                            &p,
                            |&x| neighbours(x, &seen),
                            |&x| hamming(l, x),
                            |&x| x == l,
                        )
                        .unwrap()
                        .1
                    })
                    .sum::<u64>()
            } else {
                u64::MAX
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "U5,R3,D2,L5,U4,R5,D2";
    #[test]
    fn p1_example() {
        assert_eq!(solve1(EG1), 7);
    }
    const EG2: &str = "U5,R3,D2,L5,U4,R5,D2\nU6,L1,D2,R3,U2,L1";
    #[test]
    fn p2_example() {
        assert_eq!(solve2(EG2), 32);
    }
    const EG3A: &str = "U5,R3,D2,L5,U4,R5,D2\nU6,L1,D2,R3,U2,L1";
    const EG3B: &str = "U20,L1,B1,L2,B1,R2,L1,F1,U1
U10,F1,B1,R1,L1,B1,L1,F1,R2,U1
U30,L2,F1,R1,B1,R1,F2,U1,F1
U25,R1,L2,B1,U1,R2,F1,L2
U16,L1,B1,L1,B3,L1,B1,F1";
    #[test]
    fn p3_example() {
        assert_eq!(solve3(EG3A), 5);
        assert_eq!(solve3(EG3B), 46);
    }
}
