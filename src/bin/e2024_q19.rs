use everybody_codes::{cartesian::Point, grid2d::Grid2d};
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q19_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q19_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q19_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}

fn solve<const PART: usize>(input: &str) -> String {
    let iterations = match PART {
        1 => 1,
        2 => 100,
        3 => 1048576000,
        _ => unimplemented!(),
    };
    let (key, grid) = input.split_once("\n\n").unwrap();
    let grid = Grid2d::from_str(grid, |x| x);

    let mut key_iter = key.chars().cycle();
    let rotate_grid = Grid2d::from_fn(grid.dim(), |x| {
        if grid.is_edge(x) {
            '.'
        } else {
            key_iter.next().unwrap()
        }
    });
    //step grid is a grid of points. We start with each point at it's own position, then scramble the grid as specified.
    //this gives us a where to get the new position of point P, you look at the value in position P.
    let mut step_grid = Grid2d::from_fn(grid.dim(), |x| x);
    for point in step_grid.indexes() {
        if !step_grid.is_edge(point) {
            let k = rotate_grid[point];
            let neighbours = step_grid
                .neighbours_with_diagonals(point)
                .map(|x| (x, step_grid[x]))
                .collect_vec();
            let step = if k == 'L' { 1 } else { 7 };
            for ix in 0..neighbours.len() {
                step_grid[neighbours[ix].0] = neighbours[(ix + step) % neighbours.len()].1;
            }
        }
    }
    let step_grid = step_grid;
    let final_grid = Grid2d::from_fn(grid.dim(), |p| {
        let iter = everybody_codes::iter::unfold(p, |x: &Point<usize>| step_grid[*x]);
        let final_pos =
            everybody_codes::iter::quick_index_by_simple_cycle(iter, iterations).unwrap();
        grid[final_pos]
    });

    // println!("Final grid:\n{final_grid}\n\n");
    let mut s = String::new();
    for x in final_grid
        .iter()
        .skip_while(|&&x| x != '>')
        .skip(1)
        .take_while(|&&x| x != '<')
    {
        s.push(*x);
    }
    s
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "LR

>-IN-
-----
W---<";
    #[test]
    fn p1_example() {
        assert_eq!(&solve::<1>(EG1), "WIN");
    }
    const EG2: &str = "RRLL

A.VI..>...T
.CC...<...O
.....EIB.R.
.DHB...YF..
.....F..G..
D.H........";
    #[test]
    fn p2_example() {
        assert_eq!(&solve::<2>(EG2), "VICTORY");
    }
}
