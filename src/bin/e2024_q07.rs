use everybody_codes::{cartesian::Dir, grid2d::Grid2d, iter::PermutationBag};
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q07_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q07_p2.txt");
const P2_TRACK: &str = "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-";
const P3_TRACK: &str = "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=       
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =          
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-";
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q07_p3.txt");

fn main() {
    println!("P1: {}", solve(P1_INPUT));
    println!("P2: {}", solve2(P2_INPUT, P2_TRACK));
    println!("P3: {}", solve3(P3_INPUT, P3_TRACK));
}

fn solve(input: &str) -> String {
    let track = ['S'];
    input
        .lines()
        .map(|l| {
            let (name, instructions) = l.split_once(":").unwrap();
            let instructions: Vec<char> = instructions
                .split(",")
                .map(|x| x.chars().next().unwrap())
                .collect();
            let gathered = run_race(&instructions, &track, 10);
            (name, gathered)
        })
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .map(|x| x.0)
        .collect()
}

fn make_track(track: &str) -> Vec<char> {
    let grid = Grid2d::from_str(track, |x| x);
    let start = grid.indexed_iter().find(|x| x.1 == &'S').unwrap().0;
    let mut dir = Dir::Right;
    let mut pos = start;
    let mut done = false;
    std::iter::from_fn(|| {
        if done {
            return None;
        }
        let pos_ok = |p| {
            if let Some(x) = grid.get(p) {
                *x != ' '
            } else {
                false
            }
        };
        if !pos_ok(pos.step(dir)) {
            if pos_ok(pos.step(dir.turn_left())) {
                dir = dir.turn_left();
            } else {
                dir = dir.turn_right();
            }
        }
        pos = pos.step(dir);
        let here = *grid.get(pos).unwrap();
        if here == 'S' {
            done = true;
        }
        Some(here)
    })
    .collect::<Vec<char>>()
}

fn solve2(input: &str, track: &str) -> String {
    let track = make_track(track);

    input
        .lines()
        .map(|l| {
            let (name, instructions) = l.split_once(":").unwrap();
            let instructions: Vec<char> = instructions
                .split(",")
                .map(|x| x.chars().next().unwrap())
                .collect();
            let gathered = run_race(&instructions, &track, track.len() * 10);
            (name, gathered)
        })
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .map(|x| x.0)
        .collect()
}

//4 digit, not start 4 or 5.
fn solve3(input: &str, track: &str) -> usize {
    //convert track to iterator.
    let grid = Grid2d::from_str(track, |x| x);
    let start = grid.indexed_iter().find(|x| x.1 == &'S').unwrap().0;
    let mut dir = Dir::Right;
    let mut lap_count = 0;
    let mut pos = start;
    let track_iter = std::iter::from_fn(|| {
        if lap_count > 0 {
            return None;
        }
        let pos_ok = |p| {
            if let Some(x) = grid.get(p) {
                *x != ' '
            } else {
                false
            }
        };
        if !pos_ok(pos.step(dir)) {
            if pos_ok(pos.step(dir.turn_left())) {
                dir = dir.turn_left();
            } else {
                dir = dir.turn_right();
            }
        }
        pos = pos.step(dir);
        let here = *grid.get(pos).unwrap();
        if here == 'S' {
            lap_count += 1;
        }
        Some(here)
    })
    .collect::<Vec<char>>();
    let rival_score = {
        let (_name, instructions) = input.split_once(":").unwrap();
        let instructions: Vec<char> = instructions
            .split(",")
            .map(|x| x.chars().next().unwrap())
            .collect();
        run_race(&instructions, &track_iter, track_iter.len() * 11)
    };
    println!("Rival score is {rival_score}");

    PermutationBag::new(&[('+', 5), ('-', 3), ('=', 3)], 11)
        .filter(|p| {
            let p: Vec<char> = p.iter().map(|x| **x).collect();
            run_race(&p, &track_iter, track_iter.len() * 11) > rival_score
        })
        .count()
}

fn run_race(plan: &[char], track: &[char], length: usize) -> u64 {
    let mut gathered = 0;
    let mut power = 10u64;
    let mut last_report = 0;
    let mut last_delta = 0;
    let mut pos = 0;
    let mut last_pos = pos;
    let mut last_power = power;
    let mut last_delta_delta = 0;
    loop {
        if pos == length {
            break gathered;
        }
        match (track[pos % track.len()], plan[pos % plan.len()]) {
            ('-', _) | ('=', '-') | ('S', '-') => power = power.saturating_sub(1),
            ('+', _) | ('=', '+') | ('S', '+') => power += 1,
            _ => (),
        }
        gathered += power;
        pos += 1;
        if pos % track.len() == 0 && pos % plan.len() == 0 {
            let delta = gathered - last_report;
            let delta_delta = delta - last_delta;
            let power_delta = power - last_power;
            if delta_delta == last_delta_delta {
                //stable loop found, take strides to near the end.
                let loop_size = pos - last_pos;
                let mut delta = delta;
                while pos + loop_size < length {
                    delta += delta_delta;
                    gathered += delta;
                    power += power_delta;
                    pos += loop_size;
                }
            }

            last_power = power;
            last_delta_delta = delta_delta;
            last_delta = delta;
            last_report = gathered;
            last_pos = pos;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+";
    const P2_EG_TRACK: &str = "S+===
-   +
=+=-+";
    #[test]
    fn p1_example() {
        assert_eq!(&solve(EG1), "BDCA");
    }
    #[test]
    fn p2_example() {
        assert_eq!(&solve2(EG1, P2_EG_TRACK), "DCBA");
    }
}
