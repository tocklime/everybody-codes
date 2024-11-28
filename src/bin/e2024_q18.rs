use std::collections::HashSet;
use indicatif::{ProgressBar, ProgressIterator};
use rayon::prelude::*;

use everybody_codes::{cartesian::Point, grid2d::Grid2d};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q18_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q18_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q18_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve3(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let starts: HashSet<Point<usize>> = g
        .indexed_iter()
        .filter(|(x, c)| *c == &'.' && (x.x == 0 || x.x == g.dim().x - 1))
        .map(|x| x.0)
        .collect();
    let mut targets: HashSet<Point<usize>> = g
        .indexed_iter()
        .filter(|(_x, c)| *c == &'P')
        .map(|x| x.0)
        .collect();
    let mut fringe: HashSet<Point<usize>> = starts.clone();
    let mut seen: HashSet<Point<usize>> = fringe.clone();
    let mut time = 0;
    while !targets.is_empty() && !fringe.is_empty() {
        fringe = fringe
            .into_iter()
            .flat_map(|x| g.neighbours(x))
            .filter(|n| !seen.contains(n) && g[*n] != '#')
            .collect();
        seen.extend(fringe.iter().cloned());
        targets = targets.difference(&fringe).cloned().collect();
        time += 1;
    }
    time
}
fn solve3(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let starts: Vec<Point<usize>> = g
        .indexed_iter()
        .filter(|(_x, c)| *c == &'.')
        .map(|x| x.0)
        .collect();
    let targets: HashSet<Point<usize>> = g
        .indexed_iter()
        .filter(|(_x, c)| *c == &'P')
        .map(|x| x.0)
        .collect();
    println!("Targets: {}",targets.len());
    let sl = starts.len();
    let prog = ProgressBar::new(sl as u64);
    starts.par_iter().map(|&s| {
        prog.inc(1);
        let mut targets = targets.clone();
        let mut fringe: HashSet<Point<usize>> = [s].into_iter().collect();
        let mut seen: HashSet<Point<usize>> = fringe.clone();
        let mut time = 1;
        let mut score = 0;
        while !targets.is_empty() && !fringe.is_empty() {
            fringe = fringe
                .into_iter()
                .flat_map(|x| g.neighbours(x))
                .filter(|n| !seen.contains(n) && g[*n] != '#')
                .collect();
            seen.extend(fringe.iter().cloned());
            score += fringe.intersection(&targets).count() * time;
            targets = targets.difference(&fringe).cloned().collect();
            time += 1;
        }
        if targets.is_empty() {
            score
        } else {
            usize::MAX
        }
    }).min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "##########
..#......#
#.P.####P#
#.#...P#.#
##########";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 11);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve3(EG1), 12);
    }
}
