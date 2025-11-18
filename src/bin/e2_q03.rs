use std::collections::HashSet;

use everybody_codes::{cartesian::Point, grid2d::Grid2d};
use nom::{bytes::complete::tag, multi::separated_list0, Parser};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2_q03_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2_q03_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2_q03_p3.txt");

fn main() {
    println!("P1: {}", solve1(P1_INPUT));
    println!("P2: {}", solve2(P2_INPUT));
    println!("P3: {}", solve3(P3_INPUT));
}

#[derive(Debug)]
#[allow(dead_code)]
struct Die {
    id: u32,
    rolls: usize,
    pulse: usize,
    faces: Vec<i64>,
    seed: usize,
    current_face: usize,
}
impl Die {
    fn parse<'a>() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        (
            nom::character::complete::u32,
            tag(": faces=["),
            nom::multi::separated_list1(tag(","), nom::character::complete::i64),
            tag("] seed="),
            nom::character::complete::u64,
        )
            .map(|(id, _, faces, _, seed)| Self {
                id,
                rolls: 1,
                pulse: seed as usize,
                faces,
                seed: seed as usize,
                current_face: 0,
            })
    }
    fn roll(&mut self) -> i64 {
        let spin = self.rolls * self.pulse;
        self.current_face = (self.current_face + spin) % self.faces.len();
        self.pulse += spin;
        self.pulse %= self.seed;
        self.pulse += 1 + self.rolls + self.seed;
        self.rolls += 1;
        self.faces[self.current_face]
    }
}
fn solve1(input: &str) -> usize {
    let (_, mut dice) = separated_list0(tag("\n"), Die::parse())
        .parse(input.trim())
        .unwrap();
    let mut score = 0;
    for roll in 1.. {
        let roll_all = dice.iter_mut().map(|d| d.roll()).sum::<i64>();
        score += roll_all;
        if score >= 10000 {
            return roll;
        }
    }
    unreachable!()
}
fn solve2(input: &str) -> String {
    let (dice, track) = input.split_once("\n\n").unwrap();
    let mut dice = separated_list0(tag("\n"), Die::parse())
        .parse(dice.trim())
        .unwrap()
        .1;
    let player_count = dice.len();
    let track = track
        .chars()
        .map(|x| ((x as u8) - b'0') as i64)
        .collect::<Vec<i64>>();
    let mut positions = vec![Some(0usize); player_count];
    let mut finishers = Vec::new();
    for _turn in 1.. {
        for (ix, d) in dice.iter_mut().enumerate() {
            if let Some(pos) = &mut positions[ix] {
                if *pos < track.len() {
                    let roll = d.roll();
                    if roll == track[*pos] {
                        *pos += 1;
                        if *pos >= track.len() {
                            finishers.push(format!("{}", d.id));
                            if finishers.len() == player_count {
                                return finishers.join(",");
                            }
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}
fn solve3(input: &str) -> usize {
    let (dice, grid) = input.split_once("\n\n").unwrap();
    let dice = separated_list0(tag("\n"), Die::parse())
        .parse(dice.trim())
        .unwrap()
        .1;
    let grid : Grid2d<i64> = Grid2d::from_str(grid, |x| ((x as u8) - b'0') as i64);
    let mut touched : Grid2d<bool> = Grid2d::from_elem(grid.dim(), false);
    

    for mut d in dice {
        let roll1 = d.roll();
        let mut positions : HashSet<Point<usize>> = grid.indexes().filter(|x| grid[*x] == roll1).collect();
        positions.iter().for_each(|p| touched[*p] = true);
        while !positions.is_empty() {
            let roll = d.roll();
            positions = positions.iter().flat_map(|p| {
                grid.neighbours_with_self(*p).filter(|x| grid[*x] == roll)
            }).collect();
            positions.iter().for_each(|p| touched[*p] = true);
        }
    }
    touched.iter().filter(|x| **x).count()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "1: faces=[1,2,3,4,5,6] seed=7
2: faces=[-1,1,-1,1,-1] seed=13
3: faces=[9,8,7,8,9] seed=17";
    #[test]
    fn check_dice() {
        let mut d = Die::parse()
            .parse("1: faces=[1,2,4,-1,5,7,9] seed=3")
            .unwrap()
            .1;
        let r1 = d.roll();
        dbg!(d);
        assert_eq!(r1, -1);
    }

    #[test]
    fn p1_example() {
        assert_eq!(solve1(EG1), 844);
    }
    #[test]
    fn p2_example() {
        const EG: &str = "1: faces=[1,2,3,4,5,6,7,8,9] seed=13
2: faces=[1,2,3,4,5,6,7,8,9] seed=29
3: faces=[1,2,3,4,5,6,7,8,9] seed=37
4: faces=[1,2,3,4,5,6,7,8,9] seed=43

51257284";
        assert_eq!(solve2(EG), "1,3,4,2");
    }
    #[test]
    fn p3_example() {
        const EG: &str = "1: faces=[1,2,3,4,5,6,7,8,9] seed=13

1523758297
4822941583
7627997892
4397697132
1799773472";
        assert_eq!(solve3(EG), 33);
    }
    #[test]
    fn correct_answers() {
        assert_eq!(solve1(P1_INPUT), 632);
        assert_eq!(solve2(P2_INPUT), "6,4,3,7,1,5,9,8,2");
        assert_eq!(solve3(P3_INPUT), 156924);
    }
}
