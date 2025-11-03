use everybody_codes::grid2d::Grid2d;
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q20_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q20_p2.txt");
// const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q20_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve2(P2_INPUT));
    // println!("P3: {}", solve3(P3_INPUT, 384400));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let init_height = 1000;
    let time = 100;
    let g = Grid2d::from_str(input, |x| x);
    let mut h_grid = Grid2d::from_fn(g.dim(), |x| {
        if g[x] == 'S' {
            [Some(init_height); 4]
        } else {
            [None; 4]
        }
    });
    //0: Down, 1: Left, 2: Up, 3: Right.
    for _x in 0..time {
        let new_grid = Grid2d::from_fn(g.dim(), |p| {
            //heighest we can get to here
            let mut new_heights = [None; 4];
            for (from_dir, n) in h_grid.neighbours(p).enumerate() {
                let facing_dir = (from_dir + 2) % 4;

                let h = h_grid[n]
                    .iter()
                    .enumerate()
                    .filter(|x| x.0 != from_dir)
                    .map(|x| x.1)
                    .max()
                    .unwrap();
                if let Some(h) = h {
                    let new_h = match g[p] {
                        '+' => Some(h + 1),
                        '-' => Some(h - 2),
                        '#' => None,
                        _ => Some(h - 1),
                    };
                    if let Some(new_h) = new_h {
                        new_heights[facing_dir] =
                            Some(new_h.max(new_heights[facing_dir].unwrap_or_default()));
                    }
                }
            }
            new_heights
        });
        // let best = h_grid.indexed_iter().map(|x| (x.1,x.0)).max().unwrap();
        // println!("best after {_x}:\n{}\n\n",h_grid.to_string_with(|x| if let Some(x) = x {
        //     format!("{x:04} ")
        // } else {
        //     "     ".to_string()
        // }));
        h_grid = new_grid;
    }
    let x = h_grid
        .iter()
        .flatten()
        .filter_map(|x| *x)
        .max()
        .unwrap();
    x
}
fn solve2(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    let start = g.indexed_iter().find(|x| x.1 == &'S').unwrap().0;
    let checks = ['A', 'B', 'C', 'S'];
    let check_points = checks.iter().map(|p| g.find_elem(p).unwrap()).collect_vec();
    let start_dir: Option<usize> = None;
    let (path,cost) = pathfinding::directed::astar::astar(
        &(start, 0, 10000, start_dir),
        |(p, leg, height, dir)| {
            g.neighbours(*p).enumerate().filter_map(|(d_ix, n)| {
                let facing_dir = (d_ix + 2) % 4;
                if let Some(d) = dir {
                    if d_ix == *d {
                        return None;
                    }
                }
                let (new_leg, new_h) = match g[n] {
                    '+' => (*leg, Some(height + 1)),
                    '-' => (*leg, Some(height - 2)),
                    '#' => (*leg, None),
                    'A' if *leg == 0 => (*leg + 1, Some(height - 1)),
                    'B' if *leg == 1 => (*leg + 1, Some(height - 1)),
                    'C' if *leg == 2 => (*leg + 1, Some(height - 1)),
                    _ => (*leg, Some(height - 1)),
                };
                new_h.map(|h| ((n, new_leg, h, Some(facing_dir)),1))
            }).collect_vec()
        },
        |(p,leg,_,_)| 
            // if h < 10000 { 10000 - h } else { 0} + //time to get necessary height
            check_points.get(*leg).map(|t| p.manhattan_unsigned(t)).unwrap_or_default() + //time to get to next checkpoint
            check_points.iter().skip(*leg).tuple_windows().map(|(a,b)| a.manhattan_unsigned(b)).sum::<usize>(),
        |(p, leg, height, _dir)| *p == start && *leg == 3 && *height >= 10000,
    ).unwrap();
    for (ix,(p,leg,height,dir)) in path.iter().enumerate() {
        println!("At time {ix} at {p} leg {leg} height {height}, dir {dir:?}");
    }
    cost
}

// fn solve3(input: &str, initial_height: usize) -> usize {
//     //to nearest run of +s: 
//     let remaining_height = initial_height - init_left;
//     //grid height is 12, with 3 '+'s.
//     //so each grid we lose 12, and gain 6. or lose 6.
//     let complete_cycles = remaining_height / 6;
//     let h = remaining_height % 6;
//     dbg!(complete_cycles, h);
//     complete_cycles*12 + 11 //11 is manually counted last little landing bit.

// }
#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "#....S....#
#.........#
#---------#
#.........#
#..+.+.+..#
#.+-.+.++.#
#.........#";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 1045);
    }
    const EG2: &str = "####S####
#-.+++.-#
#.+.+.+.#
#-.+.+.-#
#A+.-.+C#
#.+-.-+.#
#.+.B.+.#
#########";
    #[test]
    fn p2_example() {
        assert_eq!(solve2(EG2), 24);
    }
//     const EG3 : &str = "#......S......#
// #-...+...-...+#
// #.............#
// #..+...-...+..#
// #.............#
// #-...-...+...-#
// #.............#
// #..#...+...+..#";
    // #[test]
    // fn p3_example() {
    //     assert_eq!(solve3(EG3,     1),          1);
    //     assert_eq!(solve3(EG3,     2),          2);
    //     assert_eq!(solve3(EG3,     3),          3);
    //     assert_eq!(solve3(EG3,     4),          4);
    //     assert_eq!(solve3(EG3,     5),          5);
    //     assert_eq!(solve3(EG3,     6),          6);
    //     assert_eq!(solve3(EG3,     7),          7);
    //     assert_eq!(solve3(EG3,     8),          9);
    //     assert_eq!(solve3(EG3,     9),         10);
    //     assert_eq!(solve3(EG3,    10),         11);
    //     assert_eq!(solve3(EG3,   100),        190);
    //     assert_eq!(solve3(EG3,  1000),       1990);
    //     assert_eq!(solve3(EG3, 10000),      19990);
    //     assert_eq!(solve3(EG3,100000),     199990);
    //     assert_eq!(solve3(EG3,384400),     768790);
    // }
}
