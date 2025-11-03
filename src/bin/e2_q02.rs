use std::collections::VecDeque;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2_q02_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2_q02_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2_q02_p3.txt");

fn main() {
    println!("P1: {}", solve_line(P1_INPUT));
    println!("P2: {}", solve_circle::<100>(P2_INPUT));
    println!("P3: {}", solve_circle::<100000>(P3_INPUT));
    // println!("P2: {}", solve::<2>(P2_INPUT));
    // println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve_line(input: &str) -> usize {
    let mut bolts = "RGB".chars().cycle();
    let mut balloons = input.chars();
    let mut current_bolt = None;
    let mut bolt_count = 0;
    while let Some(b) = balloons.next() {
        if current_bolt.is_none() {
            current_bolt = bolts.next();
            bolt_count += 1;
        }
        assert!(current_bolt.is_some());
        if b != current_bolt.unwrap() {
            current_bolt = None;
        }
    }
    return bolt_count;
}
fn solve_circle<const REPS: u32>(input: &str) -> usize {
    let mut bolts = "RGB".chars().cycle();
    let len = input.len() * REPS as usize;
    assert!(len % 2 == 0);
    let balloons = (0..REPS).flat_map(|_| input.chars()).collect::<Vec<char>>();
    let mut balloons_a: VecDeque<char> = balloons[0..len / 2].iter().copied().collect();
    let mut balloons_b: VecDeque<char> = balloons[len / 2..].iter().copied().collect();
    let mut bolt_count = 0;
    while balloons_a.len() > 0 {
        let bolt = bolts.next().unwrap();
        bolt_count += 1;
        // assert_eq!(balloons_a.len(), balloons_b.len());
        if balloons_a.len() == balloons_b.len() {
            if bolt == balloons_a[0] {
                //pop 2.
                balloons_a.pop_front();
                balloons_b.pop_front();
                //they are still balanced.
            } else {
                // pop 1. then they are unbalanced. 
                balloons_a.pop_front();
            }
        } else {
            //they are unbalanced.
            balloons_a.pop_front();
        }
        while balloons_a.len() + 1 < balloons_b.len()  {
            //rebalance
            balloons_a.push_back(balloons_b.pop_front().unwrap());
        }
        // println!("{} {}", balloons_a.len(), balloons_b.len());
    }
    bolt_count + balloons_b.len()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn p1_example() {
        assert_eq!(solve_line("GRBGGGBBBRRRRRRRR"), 7);
    }

    #[test]
    fn p2_example() {
        const EG1: &str = "GGBR";
        const EG2: &str = "BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG";
        assert_eq!(solve_circle::<5>(EG1), 14);
        assert_eq!(solve_circle::<10>(EG2), 304);
        assert_eq!(solve_circle::<50>(EG2), 1464);
        assert_eq!(solve_circle::<100>(EG2), 2955);
    }
    #[test]
    fn correct_answers() {
        assert_eq!(solve_line(P1_INPUT), 131);
        assert_eq!(solve_circle::<100>(P2_INPUT), 21165);
        assert_eq!(solve_circle::<100000>(P3_INPUT), 21560375);
    }
}
