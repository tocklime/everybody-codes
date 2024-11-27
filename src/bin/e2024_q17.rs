use everybody_codes::grid2d::Grid2d;
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q17_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q17_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q17_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let mut pos = g
        .indexed_iter()
        .filter(|x| x.1 == &'*')
        .map(|x| x.0)
        .collect_vec();
    let mut in_graph = vec![pos.pop().unwrap()];
    let mut constell_sizes = vec![1];
    let max_constell_dist = if PART == 3 { 6 } else { usize::MAX };
    while !pos.is_empty() {
        let (min_dist, node_ix) = pos
            .iter()
            .enumerate()
            .map(|(ix, p)| {
                (
                    in_graph
                        .iter()
                        .map(|x| x.manhattan_unsigned(p))
                        .min()
                        .unwrap(),
                    ix,
                )
            })
            .min()
            .unwrap();
        if min_dist < max_constell_dist {
            *constell_sizes.last_mut().unwrap() += min_dist + 1;
            in_graph.push(pos.swap_remove(node_ix));
        } else {
            //nothing close enough. Start a new constellation.
            in_graph.clear();
            if let Some(x) = pos.pop() {
                in_graph.push(x);
                constell_sizes.push(1);
            }
        }
    }
    constell_sizes.into_iter().k_largest(3).product()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "*...*
..*..
.....
.....
*.*..";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 16);
    }
    const EG2: &str = ".......................................
..*.......*...*.....*...*......**.**...
....*.................*.......*..*..*..
..*.........*.......*...*.....*.....*..
......................*........*...*...
..*.*.....*...*.....*...*........*.....
.......................................";
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG2), 15624);
    }
}
