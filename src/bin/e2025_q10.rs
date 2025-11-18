use std::collections::HashMap;

use everybody_codes::{
    cartesian::Point,
    grid2d::{Grid2d, ICoord},
};
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q10_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q10_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q10_p3.txt");

fn main() {
    println!("P1: {}", solve::<1, 4>(P1_INPUT));
    println!("P2: {}", solve::<2, 20>(P2_INPUT));
    println!("P3: {}", solve::<3, 0>(P3_INPUT));
}
fn solve<const PART: usize, const MOVES: usize>(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let d = g.find_elem(&'D').unwrap();
    let mut reached = Grid2d::from_elem(g.dim(), false);
    const KNIGHT_MOVES: [ICoord; 8] = [
        Point::new(1, 2),
        Point::new(2, 1),
        Point::new(2, -1),
        Point::new(1, -2),
        Point::new(-1, -2),
        Point::new(-2, -1),
        Point::new(-2, 1),
        Point::new(-1, 2),
    ];
    match PART {
        1 => {
            let mut todo = vec![d];
            reached[d] = true;
            for _ in 0..MOVES {
                let new_todo = todo
                    .into_iter()
                    .flat_map(|p| KNIGHT_MOVES.iter().map(move |k| p + *k))
                    .filter(|x| reached.get(*x) == Some(&false))
                    .collect();
                for &x in &new_todo {
                    reached[x] = true;
                }
                todo = new_todo;
            }
            g.indexed_iter()
                .filter(|(p, x)| x == &&'S' && reached[*p])
                .count()
        }
        2 => {
            let mut sheep = g.map(|_, &a| a == 'S');
            let hides = g.map(|_, &a| a == '#');
            let mut dragon = Grid2d::from_elem(g.dim(), false);
            dragon[d] = true;
            let mut kills = 0;
            //make dragon moves lookup
            let mut dragon_poss = vec![dragon.clone()];
            for _t in 0..MOVES {
                let old = dragon_poss.last().unwrap();
                let new_d = Grid2d::from_fn(g.dim(), |p| {
                    KNIGHT_MOVES
                        .iter()
                        .any(|x| old.relative_lookup(p, *x) == Some(&true))
                });
                dragon_poss.push(new_d);
            }
            // let mut escapes = 0;
            for t in 0..MOVES {
                //eat sheep.
                for (p, s) in sheep.indexed_iter_mut() {
                    if *s && !hides[p] && dragon_poss[t + 1][p] {
                        *s = false;
                        kills += 1;
                    }
                }
                //move sheep
                // escapes += sheep.get_row(sheep.dim().y-1).iter().filter(|x|**x).count();
                let new_sheep = Grid2d::from_fn(sheep.dim(), |p| {
                    sheep
                        .relative_lookup(p, Point::new(0, -1))
                        .copied()
                        .unwrap_or_default()
                });
                sheep = new_sheep;
                //eat sheep
                for (p, s) in sheep.indexed_iter_mut() {
                    if *s && !hides[p] && dragon_poss[t + 1][p] {
                        *s = false;
                        kills += 1;
                    }
                }
            }
            kills
        }
        3 => {
            let sheep_pos: Vec<Point<usize>> = g
                .indexed_iter()
                .filter(|x| x.1 == &'S')
                .map(|x| x.0)
                .collect();
            let hides = g.map(|_, &x| x == '#');
            let start = (d, sheep_pos, true);
            let mut state: HashMap<(Point<usize>, Vec<Point<usize>>, bool), usize> = HashMap::new();
            state.insert(start, 1);
            let mut solutions = 0;
            for _turn in 0.. {
                //evolve all states, and combine.
                let mut new_states = HashMap::new();
                for ((d, sheep, sheep_turn), count) in state.into_iter() {
                    if sheep_turn {
                        let sheep_opts = sheep
                            .iter()
                            .enumerate()
                            .filter_map(|(ix, p)| {
                                let n = p.up();
                                if n != d || hides.get(n) != Some(&false) {
                                    Some(ix)
                                } else {
                                    None
                                }
                            })
                            .collect_vec();
                        if !sheep_opts.is_empty() {
                            for o in sheep_opts {
                                let mut new_sheep = sheep.clone();
                                new_sheep[o] = new_sheep[o].up();
                                if g.get(new_sheep[o]).is_none() {
                                    //sheep escaped. prune this branch.
                                } else if g.values_in_direction(new_sheep[o], Point::new(0isize,1)).all(|x| x.1 == &'#') {
                                    //sheep entered unbroken run of hides to end. prune
                                } else {
                                    *new_states.entry((d, new_sheep, false)).or_default() += count;
                                }
                            }
                        } else {
                            *new_states.entry((d, sheep, false)).or_default() += count;
                            //sheep pass their turn.
                        }
                    } else {
                        //dragon moves.
                        let dragon_opt = KNIGHT_MOVES
                            .iter()
                            .map(move |k| d + *k)
                            .filter(|x| g.get(*x).is_some())
                            .collect_vec();
                        assert!(!dragon_opt.is_empty());
                        for nd in dragon_opt {
                            let mut new_sheep = sheep.clone();
                            new_sheep.retain(|x| x != &nd || hides[*x]);
                            if new_sheep.is_empty() {
                                //found a solution.
                                solutions += count;
                            } else {
                                *new_states.entry((nd, new_sheep, true)).or_default() += count;
                            }
                        }
                    }
                }
                state = new_states;
                if state.is_empty() {
                    break;
                }
            }

            solutions
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "...SSS.......
.S......S.SS.
..S....S...S.
..........SS.
..SSSS...S...
.....SS..S..S
SS....D.S....
S.S..S..S....
....S.......S
.SSS..SS.....
.........S...
.......S....S
SS.....S..S..";
    const EG2: &str = "...SSS##.....
.S#.##..S#SS.
..S.##.S#..S.
.#..#S##..SS.
..SSSS.#.S.#.
.##..SS.#S.#S
SS##.#D.S.#..
S.S..S..S###.
.##.S#.#....S
.SSS.#SS..##.
..#.##...S##.
.#...#.S#...S
SS...#.S.#S..";
    const EG3: &str = "SSS
..#
#.#
#D.";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1, 3>(EG1), 27);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2, 3>(EG2), 27);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3, 0>(EG3), 15);
        assert_eq!(
            solve::<3, 0>(
                "SSS
..#
..#
.##
.D#"
            ),
            8
        );

        assert_eq!(
            solve::<3, 0>(
                "..S..
.....
..#..
.....
..D.."
            ),
            44
        );
        assert_eq!(
            solve::<3, 0>(
                ".SS.S
#...#
...#.
##..#
.####
##D.#"
            ),
            4406
        );
        assert_eq!(
            solve::<3, 0>(
                "SSS.S
.....
#.#.#
.#.#.
#.D.#"
            ),
            13033988838
        );
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1, 4>(P1_INPUT), 152);
        assert_eq!(solve::<2, 20>(P2_INPUT), 1753);
        assert_eq!(solve::<3, 0>(P3_INPUT), 139924465828171);
    }
}
//TODO: this is slow, probably because of hashing. Also, sheep don't need to be a vec<Point> Vec<usize> would do. If a sheep gets into an unbroken run of hides to the exit, you can prune.