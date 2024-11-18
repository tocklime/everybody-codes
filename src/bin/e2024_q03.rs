use everybody_codes::{cartesian::Point, grid2d::Grid2d};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q03_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q03_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q03_p3.txt");
fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}

fn step_grid<F, I>(g: Grid2d<usize>, neighbours: F) -> (Grid2d<usize>, usize)
where
    F: Fn(&Grid2d<usize>, Point<usize>) -> I,
    I: Iterator<Item = Point<usize>>,
{
    let mut changes = 0;
    let mut ans = g.clone();

    for (pos, val) in g.indexed_iter() {
        if *val > 0 && !g.is_edge(pos) && neighbours(&g, pos).all(|x| g[x] == *val) {
            //all neighbours low enough, can dig this out.
            ans[pos] = val + 1;
            changes += 1;
        }
    }
    (ans, changes)
}
fn solve<const PART: usize>(input: &str) -> usize {
    let mut m = everybody_codes::grid2d::Grid2d::from_str(input, |x| match x {
        '.' => 0usize,
        '#' => 1usize,
        _ => unreachable!(),
    });
    loop {
        let (n, count) = if PART == 3 {
            step_grid(m, |g, p| g.neighbours_with_diagonals(p))
        } else {
            step_grid(m, |g, p| g.neighbours(p))
        };
        m = n;
        if count == 0 {
            break;
        }
    }
    // println!("{}", m.to_string_with(|x| x.to_string()));
    m.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG: &str = "..........
..###.##..
...####...
..######..
..######..
...####...
..........";

    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG), 35);
    }

    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG), 29);
    }
}
