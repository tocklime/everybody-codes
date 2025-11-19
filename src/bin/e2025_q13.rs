const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q13_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q13_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q13_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    match PART {
        1 => {
            let ns: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
            let mut f = Vec::new();
            let mut b = Vec::new();
            f.push(1);
            for (ix, n) in ns.iter().enumerate() {
                if ix % 2 == 0 {
                    f.push(*n);
                } else {
                    b.push(*n);
                }
            }
            f.extend(b.iter().rev());
            f[2025 % f.len()]
        }
        2 | 3 => {
            let ns : Vec<(usize,usize)> = input.lines().map(|l| {
                let (a,b) = l.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            }).collect();
            let total = 1 + ns.iter().map(|(a,b)| 1 + b - a).sum::<usize>();
            let count = if PART == 2 { 20252025} else { 202520252025};
            let ix = count % total;
            // dbg!(total, ix);
            let mut f = Vec::new();
            let mut ba = Vec::new();
            f.push((1,1));
            for (ix, (a,b)) in ns.iter().enumerate() {
                if ix % 2 == 0 {
                    f.push((*a,*b));
                } else {
                    ba.push((*b,*a));
                }
            }
            f.extend(ba.iter().rev());
            let mut remaining = ix;
            for (a,b) in &f {
                let count_here = b.abs_diff(*a) + 1;
                // dbg!(a,b,count_here, remaining);
                if remaining >= count_here {
                    remaining -= count_here;
                } else {
                    if a>b {
                        //going backward from a to b.
                        return a - remaining
                    } else {
                        return a + remaining
                    }
                }
            }
            todo!()
        }
        _ => unimplemented!(),
    }
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
    // #[test]
    // fn p3_example() {
    //     assert_eq!(solve::<3>(EG3), 0);
    // }

    // #[test]
    // fn correct_answers() {
    //     assert_eq!(solve::<1>(P1_INPUT), 0);
    //     assert_eq!(solve::<2>(P2_INPUT), 0);
    //     assert_eq!(solve::<3>(P3_INPUT), 0);
    // }
}
