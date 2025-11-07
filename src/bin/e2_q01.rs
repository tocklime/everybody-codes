use everybody_codes::{
    cartesian::Point,
    grid2d::{self, Grid2d},
};
use proptest::bits::BitSetLike;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2_q01_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2_q01_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2_q01_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {:?}", solve3(P3_INPUT));
}

fn score_drop(grid: &Grid2d<char>, rules: &str, toss_slot: usize) -> usize {
    let init_x = (toss_slot - 1) * 2;
    let mut pos = Point::new(init_x, 0usize);
    let mut steps = rules.chars();
    loop {
        match grid.get(pos) {
            Some('*') => {
                pos.x = match steps.next().unwrap() {
                    'L' => {
                        if pos.x == 0 {
                            pos.x + 1
                        } else {
                            pos.x - 1
                        }
                    }
                    'R' => {
                        if pos.x + 1 == grid.dim().x {
                            pos.x - 1
                        } else {
                            pos.x + 1
                        }
                    }
                    _ => unreachable!(),
                };
                pos.y += 1;
            }
            Some(_) => {
                //just fall.
                pos.y += 1;
            }
            None => break,
        }
    }
    let last_slot = pos.x / 2 + 1;
    (last_slot * 2).saturating_sub(toss_slot)
}

fn find_unique_set(grid: &[Vec<usize>], conv: impl Fn(usize)->usize) -> usize {
    let p = pathfinding::directed::dijkstra::dijkstra(&(0,0u32), |(_, used)| {
        //have made y choices, have used the slots in `used`.
        let y = used.count_ones() as usize;
        let mut choices = Vec::new();
        for (pos, x) in grid[y].iter().enumerate() {
            let as_bit = 1 << pos;
            if as_bit & used == 0 {
                choices.push(((*x, used | as_bit), conv(*x)))
            }
        }
        choices
    }, |x| x.1.count() == grid.len()).unwrap();
    p.0.iter().map(|x| x.0).sum()
}

fn solve3(input: &str) -> String {
    let (grid, instrs) = input.split_once("\n\n").unwrap();
    let grid = grid2d::Grid2d::from_str(grid, |x| x);
    let slot_count = grid.dim().x.div_ceil(2);
    let mut ans_grid = Vec::new();
    for rules in instrs.lines() {
        let all= (1..=slot_count).map(|x| score_drop(&grid, rules, x)).collect::<Vec<_>>();
        ans_grid.push(all);
    }
    assert_eq!(ans_grid.len(), 6);
    let minimum = find_unique_set(&ans_grid, |x| x);
    ans_grid.iter_mut().for_each(|x| x.reverse());
    let max_score = *ans_grid.iter().flatten().max().unwrap();
    let maximum = find_unique_set(&ans_grid, |x| max_score - x);
    format!("{minimum} {maximum}")
}

// to get lowest score:
//  Scores for 0:RRRLRLRRRRRL: [(5, 1), (5, 3), (8, 2), (9, 7), (9, 9), (10, 4), (10, 8), (11, 5), (12, 6)]                                                                                                                                                  ▐
//  Scores for 1:LLLLRLRRRRRR: [(1, 5), (2, 4), (2, 6), (3, 3), (4, 2), (5, 1), (8, 8), (9, 7), (9, 9)]                                                                                                                                                      ▐
//  Scores for 2:RLLLLLRLRLRL: [(0, 2), (0, 3), (1, 1), (1, 5), (2, 4), (2, 6), (3, 7), (3, 9), (4, 8)]                                                                                                                                                      ▐
//  Scores for 3:LRLLLRRRLRLR: [(2, 4), (3, 3), (3, 5), (4, 2), (5, 1), (6, 6), (7, 7), (8, 8), (9, 9)]                                                                                                                                                      ▐
//  Scores for 4:LLRLLRLLLRRL: [(0, 4), (1, 3), (2, 2), (2, 6), (3, 1), (3, 5), (3, 7), (4, 8), (5, 9)]                                                                                                                                                      ▐
//  Scores for 5:LRLRLLLRRRRL: [(3, 3), (3, 5), (4, 2), (4, 4), (4, 6), (5, 1), (7, 7), (8, 8), (9, 9)]
fn solve<const PART: usize>(input: &str) -> usize {
    let (grid, instrs) = input.split_once("\n\n").unwrap();
    let grid = grid2d::Grid2d::from_str(grid, |x| x);
    if PART == 1 {
        let mut ans = 0;
        for (rules, slot) in instrs.lines().zip(1usize..) {
            ans += score_drop(&grid, rules, slot);
        }
        ans
    } else {
        let slot_count = grid.dim().x.div_ceil(2);
        let mut ans = 0;
        for rules in instrs.lines() {
            let best = (1..=slot_count)
                .map(|x| (score_drop(&grid, rules, x), x))
                .max()
                .unwrap();
            ans += best.0;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*...*..
.*.*.*.*.*...*.*.
*.*.....*...*.*.*
.*.*.*.*.*.*.*.*.
*...*...*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*
.*...*...*.*.*.*.
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.

RRRLRLRRRRRL
LLLLRLRRRRRR
RLLLLLRLRLRL
LRLLLRRRLRLR
LLRLLRLLLRRL
LRLRLLLRRRRL
LRLLLLLLRLLL
RRLLLRLLRLRR
RLLLLLRLLLRL";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 26);
    }

    #[test]
    fn p2_example() {
        const EG: &str = "*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*...*.*...*.*.*..
.*...*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.......*.
*.*.*.*.*.*.*.*.*.*...*..
.*.*.*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*.*.*....
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*...*.*.
*.*.*.*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.....*.*.
*.*.*.*.*.*.*.*...*...*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*.*.*.*.*
.*...*.*.*.*...*.*.*...*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.

RRRLLRRRLLRLRRLLLRLR
RRRRRRRRRRLRRRRRLLRR
LLLLLLLLRLRRLLRRLRLL
RRRLLRRRLLRLLRLLLRRL
RLRLLLRRLRRRLRRLRRRL
LLLLLLLLRLLRRLLRLLLL
LRLLRRLRLLLLLLLRLRRL
LRLLRRLLLRRRRRLRRLRR
LRLLRRLRLLRLRRLLLRLL
RLLRRRRLRLRLRLRLLRRL";
        assert_eq!(solve::<2>(EG), 115);
    }

    #[test]
    fn p3_example_a() {
        const EG: &str = "*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*...*..
.*.*.*.*.*...*.*.
*.*.....*...*.*.*
.*.*.*.*.*.*.*.*.
*...*...*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*
.*...*...*.*.*.*.
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.

RRRLRLRRRRRL
LLLLRLRRRRRR
RLLLLLRLRLRL
LRLLLRRRLRLR
LLRLLRLLLRRL
LRLRLLLRRRRL";
        assert_eq!(solve3(EG), "13 43");
    }
    #[test]
    fn p3_example_b() {
        const EG: &str = "*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*...*.*...*.*.*..
.*...*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.......*.
*.*.*.*.*.*.*.*.*.*...*..
.*.*.*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*.*.*....
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*...*.*.
*.*.*.*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.....*.*.
*.*.*.*.*.*.*.*...*...*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*.*.*.*.*
.*...*.*.*.*...*.*.*...*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.

RRRLLRRRLLRLRRLLLRLR
RRRRRRRRRRLRRRRRLLRR
LLLLLLLLRLRRLLRRLRLL
RRRLLRRRLLRLLRLLLRRL
RLRLLLRRLRRRLRRLRRRL
LLLLLLLLRLLRRLLRLLLL";
        assert_eq!(solve3(EG), "25 66");
    }
    #[test]
    fn p3_example_c() {
        const EG: &str = "*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*.*.*.........*.*.*.*.....*.*.*
.*.*...*.*.*.*.*.*.*.*.*.*.*...*.*.*.*.
*.*.*.*...*.*.*.*.*.....*.*.*.*...*.*..
.*...*.*...*.*.*.*.*.*.*.....*.*.*.*.*.
*.*.*.*.*.....*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*...*.*.*.*.....*.*.*.*...*.
*.*...*.*.*.*.*.*.*.*...*.*.*...*.*.*.*
.*...*.*.*.*.*.*.*.*...*.*.*.*.*.*.*.*.
*.*.*.*.*.*...*.....*.*...*...*.*.*.*.*
.*...*.*.*.*.*...*.*.*.*.*...*.*...*.*.
*.*.*.*.*...*.*.*.*.*.*.*.*...*.*.*.*.*
.*.*.*.*.*.*.*.*...*.*.*.*.*.*.*.*.*.*.
....*.*.*.*...*.*.*.*.*.*.*...*.*.*...*
.*.*.*...*.*.*.*.*...*.*.*.*.*.*.*.*...
*.*.*.*.*.*.*.....*...*...*.*.*.*.*.*.*
.*.*...*.....*.*.*.*.*.*.*...*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.*.

RRRRLLRRLLLLLLLRLLRL
RRRRRRRLRRLRRLRRRLRR
RRRLLRRRRRLRRRRRLRRR
LLLLRRLLRRLLLLLRRLLL
LRRRRLRRLRLLRLLRRLRR
RRRRRRRRLRRRRLLRRRLR";
        assert_eq!(solve3(EG), "39 122");
    }
    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 48);
        assert_eq!(solve::<2>(P2_INPUT), 1153);
        assert_eq!(solve3(P3_INPUT), "38 117");
    }
}
