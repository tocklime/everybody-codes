use everybody_codes::{cartesian::Point, grid2d::Grid2d};
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q10_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q10_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q10_p3.txt");

type P = Point<usize>;
fn main() {
    println!("P1: {}", solve1(P1_INPUT));
    println!("P2: {}", solve2(P2_INPUT));
    println!("P3: {}", solve3(P3_INPUT));
}
fn solve1(input: &str) -> String {
    let grid = Grid2d::from_str(input, |x| x);
    solve_word(&grid)
}
fn read_word(grid: &Grid2d<char>, base: P) -> Option<String> {
    let mut ans = String::new();
    for row in 0..4 {
        for col in 0..4 {
            let c = grid[base + P::new(2 + col, 2 + row)];
            if c == '.' {
                return None;
            }
            ans.push(c);
        }
    }
    Some(ans)
}
fn lone_intersection(a: &[char], b: &[char]) -> Option<char> {
    let mut found = None;
    for c_a in a {
        for c_b in b {
            if c_a == c_b {
                match found {
                    Some(x) if &x != c_a => {
                        return None;
                    }
                    _ => (),
                }
                found = Some(*c_a);
            }
        }
    }
    found
}
fn vec_difference(a: &[char], b: &[char]) -> Vec<char> {
    let mut ans = Vec::new();
    let mut to_clear = b.to_vec();
    for c_a in a {
        if let Some((ix, _)) = to_clear.iter().find_position(|x| *x == c_a) {
            to_clear.swap_remove(ix);
        } else {
            ans.push(*c_a);
        }
    }
    ans
}
fn solve_word(grid: &Grid2d<char>) -> String {
    let mut ans = String::new();

    for pos in grid.indexes() {
        let here = &grid[pos];
        if *here == '.' {
            let row: Vec<char> = grid
                .get_row(pos.y)
                .iter()
                .filter(|&&x| !".#".contains(x))
                .cloned()
                .collect();
            let col: Vec<char> = grid
                .get_col(pos.x)
                .filter(|&&x| !".#".contains(x))
                .cloned()
                .collect();
            let intersect: Option<char> = lone_intersection(&row, &col);
            assert!(intersect.is_some());
            ans.push(intersect.unwrap());
        }
    }
    ans
}
fn place_character(
    grid: &mut Grid2d<char>,
    word_base: Point<usize>,
    cell_relative_position: Point<usize>,
    value: char,
) {
    // println!("Updated at {:?}:\n{}", cell_relative_position, grid.sub_grid_copied(word_base, (8,8)).to_string_with(|x| x.to_string()));
    //can we fill in any question marks?
    let row_qs = grid
        .values_in_direction(
            word_base + P::new(0, cell_relative_position.y),
            Point::new(1usize, 0),
        )
        .take(8)
        .map(|x| (x.0, *x.1))
        .collect_vec();

    if row_qs.iter().all(|x| x.1 != value) {
        if let Some(p) = row_qs.iter().find(|x| x.1 == '?') {
            grid[p.0] = value;
        }
    }
    let col_qs = grid
        .values_in_direction(
            word_base + P::new(cell_relative_position.x, 0),
            Point::new(0usize, 1),
        )
        .take(8)
        .map(|x| (x.0, *x.1))
        .collect_vec();
    if col_qs.iter().all(|x| x.1 != value) {
        if let Some(p) = col_qs.iter().find(|x| x.1 == '?') {
            grid[p.0] = value;
        }
    }
    grid[word_base + cell_relative_position] = value;
}
fn try_solve_at(grid: &mut Grid2d<char>, base: Point<usize>) -> bool {
    let solve_base = base + Point::new(2usize, 2);
    let mut any_progress_at_all = false;
    let mut ans = String::new();
    // println!("Trying to solve\n{}", grid.sub_grid_copied(base, (8,8)).to_string_with(|x| x.to_string()));
    let mut keep_going = true;

    while keep_going {
        keep_going = false;
        for row in 0..4usize {
            let row_start = base + Point::new(0, 2 + row);
            // println!("row start is {row_start:?}");
            for col in 0..4usize {
                let col_start = base + Point::new(2 + col, 0);
                let pos = solve_base + Point::new(col, row);
                let here = &grid[pos];
                if *here == '.' {
                    // dbg!(&row_start);
                    let row_given: Vec<char> = [
                        grid[row_start],
                        grid[row_start + P::new(1, 0)],
                        grid[row_start + P::new(6, 0)],
                        grid[row_start + P::new(7, 0)],
                    ]
                    .to_vec();
                    let col_given: Vec<char> = [
                        grid[col_start],
                        grid[col_start + P::new(0, 1)],
                        grid[col_start + P::new(0, 6)],
                        grid[col_start + P::new(0, 7)],
                    ]
                    .to_vec();
                    let intersect = lone_intersection(&row_given, &col_given);
                    match intersect {
                        Some(x) if x != '?' => {
                            grid[pos] = x;
                            ans.push(x);
                            keep_going = true;
                            any_progress_at_all = true;
                        }
                        _ => {
                            let row_known: Vec<char> = grid
                                .values_in_direction(
                                    solve_base + Point::new(0, row),
                                    Point::new(1usize, 0),
                                )
                                .take(4)
                                .map(|x| *x.1)
                                .collect();
                            let col_known: Vec<char> = grid
                                .values_in_direction(
                                    solve_base + Point::new(col, 0),
                                    Point::new(0usize, 1),
                                )
                                .take(4)
                                .map(|x| *x.1)
                                .collect();
                            let left_on_row = vec_difference(&row_given, &row_known);
                            if left_on_row.len() == 1 && left_on_row[0] != '?' {
                                place_character(
                                    grid,
                                    base,
                                    Point::new(2 + col, 2 + row),
                                    left_on_row[0],
                                );
                                ans.push(left_on_row[0]);
                                keep_going = true;
                                any_progress_at_all = true;
                            } else {
                                let left_on_col = vec_difference(&col_given, &col_known);
                                if left_on_col.len() == 1 && left_on_col[0] != '?' {
                                    place_character(
                                        grid,
                                        base,
                                        Point::new(2 + col, 2 + row),
                                        left_on_col[0],
                                    );
                                    ans.push(left_on_col[0]);
                                    keep_going = true;
                                    any_progress_at_all = true;
                                }
                            }
                        }
                    }
                } else {
                    ans.push(*here);
                }
            }
        }
    }
    any_progress_at_all
}

fn solve2(input: &str) -> usize {
    let mut ans = 0;
    for input_row in input.split("\n\n") {
        let row_grid = Grid2d::from_str(input_row, |x| x);
        for x in 0..15 {
            let base_point = Point::new(x * 9, 0);
            let this_grid = row_grid.sub_grid_copied(base_point, (8, 8));
            ans += word_power(&solve_word(&this_grid))
        }
    }
    ans
}
fn solve3(input: &str) -> usize {
    let mut grid = Grid2d::from_str(input, |x| x);
    let mut keep_going = true;
    while keep_going {
        keep_going = false;
        for row in 0..10 {
            for col in 0..20 {
                let base = P::new(col * 6, row * 6);
                if try_solve_at(&mut grid, base) {
                    keep_going = true;
                }
            }
        }
    }

    // println!("Final grid:\n{}", grid);

    let mut ans = 0;
    for row in 0..10 {
        for col in 0..20 {
            let base = P::new(col * 6, row * 6);
            if let Some(w) = read_word(&grid, base) {
                ans += word_power(&w);
            }
        }
    }
    ans
}

fn word_power(input: &str) -> usize {
    input
        .bytes()
        .zip(1..)
        .map(|(c, ix)| ((1 + c - b'A') as usize) * ix)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**";
    #[test]
    fn p1_example() {
        assert_eq!(&solve1(EG1), "PTBVRCZHFLJWGMNS");
    }
    #[test]
    fn p2_example() {
        assert_eq!(word_power("PTBVRCZHFLJWGMNS"), 1851);
    }
}
