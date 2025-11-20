use std::collections::HashMap;

use everybody_codes::{cartesian::Point, grid2d::Grid2d};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q14_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q14_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q14_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve3(P3_INPUT));
}
fn solve3(input: &str) -> usize {
    const N : usize = 1000000000;
    let mut seen = HashMap::new();
    let reference = Grid2d::from_str(input, |x| x == '#');
    let offset = (34 - reference.dim().x) / 2;
    let base = Point::new(offset,offset);
    let mut g = Grid2d::from_elem((34,34), false);
    let mut total_sum = 0;
    let mut round = 0;
    while round < N {
        // let new_g = 
        let new_g = step_grid(&g);
        if let Some(n) = seen.insert(new_g.clone(), round) {
            assert_eq!(n, 0);
            println!("Total is {total_sum}");
            let loops = N / round;
            let extra = N % round;
            total_sum *= loops;
            round = round * loops;
            seen.clear()
            // panic!("Seen this grid already at turn {n} now its {round}");

        }
        if new_g.sub_grid_copied(base, reference.dim()) == reference {
            total_sum += new_g.iter().filter(|x| **x).count();
        }
        g = new_g;
        round += 1;
    }
    total_sum
}
fn step_grid(g: &Grid2d<bool>) -> Grid2d<bool> {
    Grid2d::from_fn(g.dim(), |p| {
        let ns = g.diagonals_and_self(p).filter(|x| g[*x]).count() % 2 == 0;
        ns
    })
}
fn solve<const PART: usize>(input: &str) -> usize {
    let n = match PART {
        1 => 10,
        2 => 2025,
        _ => unimplemented!(),
    };
    let g = Grid2d::from_str(input, |x| x == '#');
    // println!("{}", g.to_string_with(|x| (if *x {"#"} else {"."}).into()));
    (0..n)
        .scan(g, |g, _a| {
            *g = step_grid(g);
            Some(g.iter().filter(|x| **x).count())
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = ".#.##.
##..#.
..##.#
.#.##.
.###..
###.##";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 200);
    }
    // #[test]
    // fn p2_example() {
    //     assert_eq!(solve::<2>(EG2), 0);
    // }
    #[test]
    fn p3_example() {
        assert_eq!(solve3("#......#
..#..#..
.##..##.
...##...
...##...
.##..##.
..#..#..
#......#"), 278388552);
    }

    // #[test]
    // fn correct_answers() {
    //     assert_eq!(solve::<1>(P1_INPUT), 0);
    //     assert_eq!(solve::<2>(P2_INPUT), 0);
    //     assert_eq!(solve::<3>(P3_INPUT), 0);
    // }
}
