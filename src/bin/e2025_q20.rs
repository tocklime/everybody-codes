use everybody_codes::{
    cartesian::Point,
    grid2d::{Coord, Grid2d},
};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q20_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q20_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q20_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}

fn tri_neighbours<'a>(g: &'a Grid2d<char>, pos: Coord) -> impl Iterator<Item = Coord> + use<'a> {
    let flat_top = !(pos.x + pos.y).is_multiple_of(2);
    let ns = if flat_top {
        [pos.up(), pos.left(), pos.right()]
    } else {
        [pos.down(), pos.left(), pos.right()]
    };
    ns.into_iter().filter(|p| g.get(*p).is_some())
}
fn rotate_grid(g: &Grid2d<char>) -> Grid2d<char> {
    let mut ng = Grid2d::from_elem(g.dim(), '.');
    let mut ng_pos = Point::new(0, 0);
    let tri_size = g.dim().x;
    for row_start_x in (0..tri_size).step_by(2).rev() {
        //from this start point, take this and left neighbour from successive rows.
        let mut g_pos = Point::new(row_start_x, 0);
        for _ in 0..row_start_x / 2 {
            ng[ng_pos] = g[g_pos];
            ng_pos.x += 1;
            g_pos = g_pos.left();
            ng[ng_pos] = g[g_pos];
            ng_pos.x += 1;
            g_pos = g_pos.up();
        }
        ng[ng_pos] = g[g_pos];
        ng_pos.x += 1;
        ng_pos.y += 1;
        ng_pos.x = ng_pos.y;
    }
    ng
}
fn solve<const PART: usize>(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    match PART {
        1 => {
            g.indexed_iter()
                .map(|(ix, c)| {
                    if *c == 'T' {
                        let ns = tri_neighbours(&g, ix);

                        ns.into_iter().filter(|x| g[*x] == 'T').count()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
                / 2
        }
        2 => {
            let s = g.find_elem(&'S').unwrap();
            let e = g.find_elem(&'E').unwrap();
            let path = pathfinding::directed::bfs::bfs(
                &s,
                |p| tri_neighbours(&g, *p).filter(|n| g[*n] == 'T' || g[*n] == 'E'),
                |p| *p == e,
            )
            .unwrap();
            path.len() - 1
        }
        3 => {
            let s = g.find_elem(&'S').unwrap();
            let g2 = rotate_grid(&g);
            let g1 = rotate_grid(&g2);
            let gs = [g, g1, g2];
            let path = pathfinding::directed::bfs::bfs(
                &(s, 0),
                |(p, grid_ix)| {
                    let new_ix = (grid_ix + 1) % 3;
                    let g = &gs[new_ix];
                    tri_neighbours(g, *p)
                        .chain([*p])
                        .filter(|&p| "TE".contains(g[p]))
                        .map(move |p| (p, new_ix))
                },
                |&(p, gix)| gs[gix][p] == 'E',
            )
            .unwrap();
            path.len() - 1
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "T#TTT###T##
.##TT#TT##.
..T###T#T..
...##TT#...
....T##....
.....#.....";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 7);
    }
    #[test]
    fn p1_example_2() {
        assert_eq!(
            solve::<1>(
                "T#T#T#T#T#T
.T#T#T#T#T.
..T#T#T#T..
...T#T#T...
....T#T....
.....T....."
            ),
            0
        );
    }
    #[test]
    fn p1_example_3() {
        assert_eq!(
            solve::<1>(
                "T#T#T#T#T#T
.#T#T#T#T#.
..#T###T#..
...##T##...
....#T#....
.....#....."
            ),
            0
        );
    }
    #[test]
    fn p2_example() {
        assert_eq!(
            solve::<2>(
                "TTTTTTTTTTTTTTTTT
.TTTT#T#T#TTTTTT.
..TT#TTTETT#TTT..
...TT#T#TTT#TT...
....TTT#T#TTT....
.....TTTTTT#.....
......TT#TT......
.......#TT.......
........S........"
            ),
            32
        );
    }
    #[test]
    fn test_rotate() {
        let g1 = "1234567
.89ABC.
..DEF..
...G...";
        let g2 = "76CBFEG
.54A9D.
..328..
...1...";
        let gg1 = Grid2d::from_str(g1, |x| x);
        let rot = rotate_grid(&gg1);
        let rot_to_s = rot.to_string();
        assert_eq!(rot_to_s.trim(), g2);
    }
    #[test]
    fn p3_example() {
        assert_eq!(
            solve::<3>(
                "T####T#TTT##T##T#T#
.T#####TTTT##TTT##.
..TTTT#T###TTTT#T..
...T#TTT#ETTTT##...
....#TT##T#T##T....
.....#TT####T#.....
......T#TT#T#......
.......T#TTT.......
........TT#........
.........S........."
            ),
            23
        );
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 120);
        assert_eq!(solve::<2>(P2_INPUT), 601);
        assert_eq!(solve::<3>(P3_INPUT), 462);
    }
}
