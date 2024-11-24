use std::{collections::{BTreeSet, HashMap}, hash::Hash};

use everybody_codes::{cartesian::Point, grid2d::Grid2d, numset::NumSet};
use itertools::{all, Itertools};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q15_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q15_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q15_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve2(P2_INPUT));
    println!("P3: {}", solve2(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let start = g.indexed_iter().find(|x| x.1 == &'.').unwrap().0;
    let path = pathfinding::directed::bfs::bfs(
        &start,
        |x| g.neighbours(*x).into_iter().filter(|x| g[*x] != '#'),
        |x| g[*x] == 'H',
    )
    .unwrap();
    dbg!(&start);
    // for c in &path {
    //     println!(" -> {}", c);
    // }
    (path.len() - 1) * 2
}
fn solve2(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let start = g.indexed_iter().find(|x| x.1 == &'.').unwrap().0;
    let start_collected: NumSet<u32> = NumSet::new();
    let all_herbs : NumSet<u32> = g.iter().filter(|x| char::is_alphabetic(**x)).map(|x| (*x as u8) - b'A').collect();
    let interesting = g.indexed_iter().filter(|(p,b)| p == &start || b.is_alphabetic() ).collect_vec();
    let mut lookup : HashMap<Point<usize>,HashMap<Point<usize>,usize>> = HashMap::new();
    for a in interesting.iter().combinations(2) {
        let b = a[1].0;
        let a = a[0].0;
        let dist = pathfinding::directed::astar::astar(&a,|p| g.neighbours(*p).filter(|n| !"#~".contains(g[*n])).map(|n| (n,1)),|x| x.manhattan_unsigned(&b), |x| *x == b);
        let d = dist.unwrap().1;
        *lookup.entry(a).or_default().entry(b).or_default() = d;
        *lookup.entry(b).or_default().entry(a).or_default() = d;
        // lookup.entry(b).or_default().entry(a).or_default().insert(dist);
    };
    let mut herb_to_locs : HashMap<u8,Vec<Point<usize>>> = HashMap::new();
    for i in &interesting {
        herb_to_locs.entry((*i.1 as u8) - b'A').or_default().push(i.0);
    }
    println!("Done lookup prep");
    let path = pathfinding::directed::astar::astar(
        &(start,start_collected),
        |(x, got)| {
            let valid_targets = if got == &all_herbs {
                assert_ne!(x, &start);
                // println!("Got all targets at {x}: {got:?}");
                vec![(start,'.')]
            } else {
                interesting.iter().filter(|x| x.1.is_alphabetic() && !got.contains((*x.1 as u8) - b'A')).map(|x| (x.0,*x.1)).collect()
            };
            valid_targets.into_iter().map(|t| {
                let mut new_got = *got;
                if t.1 != '.' {
                    new_got.insert((t.1 as u8) - b'A');
                }
                ((t.0,new_got), lookup[x][&t.0])
            }).collect_vec()
        },
        |(pos,got)|{
            if got == &all_herbs {
                //just route home. 
                if pos == &start {
                    0
                } else {
                    lookup[pos][&start]
                }
            } else {
                //find furthest thing we haven't got. (assume if we go to the nearest one, we have picked up everything else we need on the way; then go home).
                let missing = all_herbs - *got;
                let (there, last_point) = missing.iter().map(|i|{
                    herb_to_locs[&i].iter().map(|p| (lookup[pos][p],p)).min().unwrap()
                    // interesting.iter().filter(|x| x.1.is_alphabetic() && (*x.1 as u8) - b'A' == i).map(|x| (lookup[pos][&x.0],x.0)).min().unwrap()
                }).max().unwrap();
                there + lookup[&last_point][&start]
            }

        },
        |(pos, got)| pos == &start && got == &all_herbs,
    ).unwrap();

    path.1
}
// fn solve3(input: &str) -> usize {
//     let g = Grid2d::from_str(input, |x| x);
//     let start = g.indexed_iter().find(|x| x.1 == &'.').unwrap().0;
//     let start_collected: NumSet<u32> = NumSet::new();
//     let all_herbs : NumSet<u32> = g.iter().filter(|x| char::is_alphabetic(**x)).map(|x| (*x as u8) - b'A').collect();

// }

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "#####.#####
#.........#
#.######.##
#.........#
###.#.#####
#H.......H#
###########";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 26);
    }
    const EG2: &str = "##########.##########
#...................#
#.###.##.###.##.#.#.#
#..A#.#..~~~....#A#.#
#.#...#.~~~~~...#.#.#
#.#.#.#.~~~~~.#.#.#.#
#...#.#.B~~~B.#.#...#
#...#....BBB..#....##
#C............#....C#
#####################";
    #[test]
    fn p2_example() {
        assert_eq!(solve2(EG2), 38);
    }
}
