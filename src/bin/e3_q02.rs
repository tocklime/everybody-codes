use std::collections::BTreeSet;

use everybody_codes::{
    cartesian::{Dir, Point},
    grid2d::Grid2d,
};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q02_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q02_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e3_q02_p3.txt");

fn main() {
    println!("P1: {}", p1(P1_INPUT));
    println!("P2: {}", p2(P2_INPUT));
    println!("P3: {}", p3(P3_INPUT));
}
fn p1(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    let start = g.find_elem(&'@').unwrap();
    let mut pos = start;
    let mut dirs = [Dir::Down, Dir::Right, Dir::Up, Dir::Left]
        .into_iter()
        .cycle();
    let mut steps = 1;
    loop {
        let next_dir = dirs.next().unwrap();
        let next_pos = match g.get(pos.step(next_dir)) {
            None => panic!(),
            Some('.') => Some(pos.step(next_dir)),
            Some('#') => return steps,
            _ => None,
        };
        if let Some(np) = next_pos {
            g[pos] = '+';
            g[np] = '@';
            steps += 1;
            pos = np;
        }
    }
}
fn try_fill(grid: &mut Grid2d<char>, start: Point<usize>) {
    if let Some('.') = grid.get(start) {
        //this is ok, only if we can reach the edge of the grid from here.
        let mut fringe = vec![start];
        let mut seen = BTreeSet::new();
        while let Some(p) = fringe.pop() {
            if seen.insert(p) {
                for d in Dir::all_dirs() {
                    match grid.get(p.step(d)) {
                        None => return,
                        Some('.') => fringe.push(p.step(d)),
                        _ => {}
                    }
                }
            }
        }
        for p in seen {
            grid[p] = 'X';
        }
    }
}
fn p2(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    let start = g.find_elem(&'@').unwrap();
    let mut target = g.find_elem(&'#').unwrap();
    let mut pos = start;
    let mut dirs = [Dir::Down, Dir::Right, Dir::Up, Dir::Left]
        .into_iter()
        .cycle();
    let mut steps = 0;
    loop {
        let next_dir = dirs.next().unwrap();
        if g.get(pos.step(next_dir)).is_none() {
            //trying to leave the graph. this is allowed.
            let p1 = Point::new(1, 1);
            let new_size = g.dim() + Point::new(2usize, 2);
            let new_g = Grid2d::from_fn(new_size, |p| {
                if p.x == 0 || p.y == 0 {
                    '.'
                } else {
                    g.get(p - p1).copied().unwrap_or('.')
                }
            });
            g = new_g;
            target += p1;
            pos += p1;
        }
        let next_pos = match g.get(pos.step(next_dir)) {
            None => panic!("We just checked for this and expanded the grid"),
            Some('.') => Some(pos.step(next_dir)),
            _ => None,
        };
        if let Some(np) = next_pos {
            g[pos] = '+';
            g[np] = '@';
            steps += 1;
            pos = np;

            //try to flood fill each adjacent cell.
            for d in Dir::all_dirs() {
                try_fill(&mut g, np.step(d));
            }
            if Dir::all_dirs()
                .iter()
                .all(|d| g.get(target.step(*d)) != Some(&'.'))
            {
                return steps;
            }
        }
    }
}
fn p3(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    let start = g.find_elem(&'@').unwrap();
    let mut targets: Vec<Point<usize>> = g
        .indexed_iter()
        .filter(|x| x.1 == &'#')
        .map(|x| x.0)
        .collect();
    for p in g.indexes() {
        if g[p] == '.' {
            try_fill(&mut g, p);
        }
    }
    let mut pos = start;
    let mut dirs = [
        Dir::Down,
        Dir::Down,
        Dir::Down,
        Dir::Right,
        Dir::Right,
        Dir::Right,
        Dir::Up,
        Dir::Up,
        Dir::Up,
        Dir::Left,
        Dir::Left,
        Dir::Left,
    ]
    .into_iter()
    .cycle();
    let mut steps = 0;
    let mut offset = Point::new(0, 0);
    loop {
        let next_dir = dirs.next().unwrap();
        if g.get(pos.step(next_dir)).is_none() {
            //trying to leave the graph. this is allowed.
            let p1 = Point::new(1, 1);
            let new_size = g.dim() + Point::new(2usize, 2);
            let new_g = Grid2d::from_fn(new_size, |p| {
                if p.x == 0 || p.y == 0 {
                    '.'
                } else {
                    g.get(p - p1).copied().unwrap_or('.')
                }
            });
            g = new_g;
            offset += p1;
            pos += p1;
        }
        let next_pos = match g.get(pos.step(next_dir)) {
            None => panic!(),
            Some('.') => Some(pos.step(next_dir)),
            _ => None,
        };
        if let Some(np) = next_pos {
            g[pos] = '+';
            g[np] = '@';
            steps += 1;
            pos = np;

            //try to flood fill each adjacent cell.
            for d in Dir::all_dirs() {
                try_fill(&mut g, np.step(d));
            }

            targets.retain(|&target| {
                Dir::all_dirs()
                    .iter()
                    .any(|d| g.get((target + offset).step(*d)) == Some(&'.'))
            });

            if targets.is_empty() {
                return steps;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = ".......
.......
.......
.#.@...
.......
.......
.......";
    #[test]
    fn p1_example() {
        assert_eq!(p1(EG1), 12);
    }
    const EG2: &str = ".......
.......
.......
.#.@...
.......
.......
.......";
    #[test]
    fn p2_example() {
        assert_eq!(p2(EG2), 47);
    }
    const EG3: &str = "#..#.......#...
...#...........
...#...........
#######........
...#....#######
...#...@...#...
...#.......#...
...........#...
...........#...
#..........#...
##......#######";
    const EG3A: &str = "................................................................
.........................###.........###........................
....................##...###########.#####......#.......###.....
.........##.............############....####.............##.....
.......######..............#############.###....................
.........##................#############.###.......##...........
...............##...........########....####....................
...............................####.#######...........##........
........................##################...........####.......
....#.........#########################.....##......######......
..............#.##......##....##..##.##...............##........
..............................##....##..........##..............
........####....#################..######...................##..
........###.....###...####..###..##...##.########...............
.................####....###..##.##.##..###....##.....##........
....##...........#######.....##..##..##......#####..........#...
...........##......#########......#....##.######..........#####.
...........##........###########################....#.......#...
.........######............##################.......#...........
...........##.............#########.............................
............#.........#############....................#........
.....#...........##..####......###......##........#.............
.............##................###..........#.....#.............
..................##...........##...................##..........
..........................###.####.####.........................
................#.###########..###.############.#...............
.....#####....###...............................###.............
.....#####...#############......@......#############............
.....#########.###################################.#............
...###########..##.....###################.....##..##...........
...######...#######.##...###.........##...##...###.##...........
.....##.########........#####..###..####.......#.########.......
............#########################################...........
..............#####################################.............
...............................###..............................
................................................................";
    #[test]
    fn p3_example() {
        assert_eq!(p3(EG2), 87);
        assert_eq!(p3(EG3), 239);
        assert_eq!(p3(EG3A), 1539);
    }

    #[test]
    fn correct_answers() {
        assert_eq!(p1(P1_INPUT), 284);
        assert_eq!(p2(P2_INPUT), 3645);
        assert_eq!(p3(P3_INPUT), 2427);
    }
}
