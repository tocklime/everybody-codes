use std::cmp::Ordering;

use num::Integer;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q11_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q11_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q11_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn do_step(flock: &mut [usize], c: Ordering) -> usize {
    let mut any_move = 0;
    for ix in 0..flock.len() - 1 {
        if flock[ix].cmp(&flock[ix+1]) == c {
            match c {
                Ordering::Greater => {
                    flock[ix].dec();
                    flock[ix+1].inc();
                }
                Ordering::Less => {
                    flock[ix+1].dec();
                    flock[ix].inc();
                },
                Ordering::Equal => panic!(),
            }
            any_move.inc();
        }
    }
    any_move
}
fn solve<const PART: usize>(input: &str) -> usize {
    let mut flock : Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut step = 1;
    for round in 0.. {
        if step == 1 {
            if do_step(&mut flock, Ordering::Greater) == 0 {
                step = 2;
            }
        }
        if step == 2 {
            if PART >= 2 {
                //We know that the list is sorted now (otherwise step one would not be finished)
                //now swapping back the other way, each round has the ultimate effect of incrementing one column, and decrementing another.
                //so, the total remaining rounds is sum of difference from average divided by two (plus however many rounds step 1 took).
                let mean = flock.iter().sum::<usize>() / flock.len();
                let total_to_move = flock.iter().map(|x| x.abs_diff(mean)).sum::<usize>();
                return round + (total_to_move / 2);
            }
            do_step(&mut flock, Ordering::Less);
        }
        if PART == 1 && round == 9 {
            return flock.iter().zip(1..).map(|(count, ix)| count * ix).sum()
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "9
1
1
4
9
6";
    const EG2: &str = "805
706
179
48
158
150
232
885
598
524
423";
    const EG3: &str = "1
2
3
4
5
6
7
8
9
15";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 109);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>(EG1), 11);
        assert_eq!(solve::<2>(EG2), 1579);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG3), 15);
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 298);
        assert_eq!(solve::<2>(P2_INPUT), 3069290);
        assert_eq!(solve::<3>(P3_INPUT), 122112328902856);
    }
}
