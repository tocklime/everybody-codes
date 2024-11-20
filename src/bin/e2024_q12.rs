use everybody_codes::{cartesian::Point, grid2d::Grid2d};
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q12_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q12_p2.txt");
// const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q12_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    // println!("P3: {}", solve::<3>(P3_INPUT));
}
fn make_shot(from: Point<isize>, power: isize) -> Point<isize> {
    from + Point::<isize>::new(power,-power) + Point::new(power,0)
}
fn hits(fall_point: Point<isize>, target: Point<isize>) -> bool {
    //fall in a line y=x
    let diff = target - fall_point;
    diff.x == diff.y
}
fn solve<const PART: usize>(input: &str) -> isize {
    let g = Grid2d::from_str(input, |x| x);
    // println!("{g}");
    let catapults = g.indexed_iter().filter(|x| "ABC".contains(*x.1)).map(|x| (x.0.into(),x.1)).collect_vec();
    //natural order of targets here is good: top to bottom, left to right.
    let targets = g.indexed_iter().filter(|x| "TH".contains(*x.1)).map(|x| (x.0.into(),x.1,false)).collect_vec();
    let mut ans = 0;
    let max_target_x : isize = targets.iter().map(|x: &(Point<isize>, &char, bool)| x.0.x).max().unwrap();
    let min_catapult_x : isize = catapults.iter().map(|x: &(Point<isize>, &char) | x.0.x).min().unwrap();
    let max_range = max_target_x - min_catapult_x;
    let max_power = max_range / 2 + 1;
    dbg!(min_catapult_x,max_target_x,max_range,max_power);
    for t in targets {
        let shot = (1..max_power).find_map(|p| {
            let n = catapults.iter().find_map(|c| {
                let s = make_shot(c.0, p);
                if hits(s,t.0) {
                    let mult = if *t.1 == 'H' { 2} else {1};
                    Some(((*c.1 as u8) - b'A' + 1) as isize * p * mult)
                } else {
                    None
                }
            });
            n
        });
        if let Some(shot) = shot {
            ans += shot;
        } else {
            panic!("Cannot make shot to hit target {t:?}");
        }
    }
    ans
}


#[cfg(test)]
mod test {
    use super::*;
    const EG1 : &str = ".............
.............
.C...........
.B......T....
.A......T.T..
=============";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 13);
    }
}