use everybody_codes::{cartesian::Point, grid2d::Grid2d};
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q12_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q12_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q12_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve3(P3_INPUT));
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
fn find_firing_solution(target: Point<isize>, delay_time: isize) -> Option<isize>{
    let real_target = target + Point::new(-1,-1)*delay_time;
    assert_eq!(real_target.x%2,0);
    let time_to_intercept = real_target.x / 2;
    let intercept_point = real_target + Point::new(-1,-1) * time_to_intercept;
    assert!(intercept_point.y >= 0);
    // println!("Trying to hit {target} after delaying for {delay_time}. Target will be at {real_target}. At intercept time it will be at {intercept_point}. Time of flight is {time_to_intercept}");
    (0..3).filter_map(|c| -> Option<isize>{
        let height_diff = intercept_point.y - c;
        //3 phases of a shot. either we hit on the ascendant, the plateau, or the fall.
        //if height_diff > time_to_intercept, we can't hit it.
        if height_diff > time_to_intercept {
            None //too high
        } else if 2 * height_diff > time_to_intercept {
            // println!("Found solution with power {height_diff} from catapult {c}: score is {}", (c+1)*height_diff);
            Some((c+1)*height_diff) //on ascendant or plateau.
        } else {
            let x = height_diff + time_to_intercept;
            if 0 == x % 3 {
                // println!("Found solution on descendant with power {} from catapult {c}: score is {}", x/3, (c+1)*x/3);
                Some((c+1) * x/3)
            } else {
                None
            }
        }
    }).min()
}

fn solve3(input: &str) -> isize {
    input.lines().map(|l| {
        let (x, y) = l.split_once(' ').unwrap();
        let p = Point::<isize>::new(x.parse().unwrap(),y.parse().unwrap());
        find_firing_solution(p, p.x%2).expect("we can hit everything on t=0 or t=1")
    }).sum()
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
    const EG3: &str = "6 5
6 7
10 5";
    #[test]
    fn p3_example(){
        assert_eq!(solve3(EG3), 11);
    }
}