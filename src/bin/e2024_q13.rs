use everybody_codes::grid2d::Grid2d;
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q13_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q13_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q13_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let grid = Grid2d::from_str(input, |x| x);
    let start = grid.indexed_iter().find(|x| x.1 == &'E').unwrap().0;
    let targets = grid
        .indexed_iter()
        .filter(|x| x.1 == &'S')
        .map(|x| x.0)
        .collect_vec();
    let ans = pathfinding::directed::dijkstra::dijkstra(
        &(start, 0u8),
        |&(p, level)| {
            grid.neighbours(p)
                .filter(|n| grid[*n] != '#')
                .map(|n| {
                    let new_level = match grid[n] {
                        'S' | 'E' => 0,
                        x => x as u8 - b'0',
                    };
                    let diff = level.abs_diff(new_level);
                    let diff = diff.min(10 - diff);
                    ((n, new_level), diff as usize + 1)
                })
                .collect_vec()
        },
        |&x| targets.contains(&x.0),
    );
    ans.unwrap().1
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "#######
#6769##
S50505E
#97434#
#######";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 28);
    }
}
