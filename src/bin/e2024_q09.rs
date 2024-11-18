use everybody_codes::collections::VecLookup;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q09_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q09_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q09_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> u32 {
    let ns: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut dp = match PART {
        1 => Dp::new(&[1, 3, 5, 10]),
        2 => Dp::new(&[1, 3, 5, 10, 15, 16, 20, 24, 25, 30]),
        3 => Dp::new(&[
            1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
        ]),
        _ => unreachable!(),
    };
    match PART {
        1 | 2 => ns.iter().map(|&n| dp.get(n as usize)).sum(),
        3 => ns.iter().map(|&n| dp.split_and_search(n as usize)).sum(),
        _ => unimplemented!(),
    }
}

struct Dp {
    data: VecLookup<u32>,
    stamps: Vec<u32>,
    correct_up_to: usize,
}
impl Dp {
    fn new(stamps: &[u32]) -> Self {
        let mut data = VecLookup::with_capacity(1000);
        data.insert(0, 0);
        Self {
            data,
            stamps: stamps.to_vec(),
            correct_up_to: 0,
        }
    }
    fn make_correct_to(&mut self, target: usize) {
        for n in self.correct_up_to..=target {
            let stamp_count = self.data[n] + 1;
            for &s in &self.stamps {
                let x = self
                    .data
                    .entry(n + (s as usize))
                    .or_insert_with(|| stamp_count);
                if *x > stamp_count {
                    *x = stamp_count;
                }
            }
        }
        self.correct_up_to = target;
    }
    fn get(&mut self, target: usize) -> u32 {
        self.make_correct_to(target);
        self.data[target]
    }
    fn split_and_search(&mut self, target: usize) -> u32 {
        let a_1 = target / 2 - 50;

        (0..=101)
            .map(|add| {
                let a = a_1 + add;
                let b = target - a;
                if a.abs_diff(b) <= 100 {
                    self.get(a) + self.get(b)
                } else {
                    u32::MAX
                }
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "2
4
7
16";
    const EG2: &str = "33
41
55
99";
    const EG3: &str = "156488
352486
546212";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 10);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>(EG2), 10);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG3), 10449);
    }
}
