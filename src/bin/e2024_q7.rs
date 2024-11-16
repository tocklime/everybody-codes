use everybody_codes::{cartesian::Dir, grid2d::Grid2d};
use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q07_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q07_p2.txt");
const P2_TRACK: &str  = "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
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
    let track_len = 10;
    let foo = input.lines().map(|l| {
        let (name, instructions) = l.split_once(":").unwrap();
        let mut instructions = instructions.split(",").cycle();
        let mut gathered = 0;
        let mut power = 10u32;
        for _ in 0..track_len {
            match instructions.next() {
                Some("=") => (),
                Some("+") => power += 1,
                Some("-") => power = power.saturating_sub(1),
                _ => unreachable!(),
            }
            gathered += power;
        }
        (name, gathered)
    }).sorted_by(|a,b| b.1.cmp(&a.1)).collect::<Vec<_>>();
    foo.into_iter().map(|x| x.0).collect()
}

fn solve2(input: &str, track: &str) -> String {
    //convert track to iterator.
    let grid = Grid2d::from_str(track, |x| x);
    let start = grid.indexed_iter().find(|x| x.1 == &'S').unwrap().0;
    let mut dir = Dir::Right;
    let mut lap_count = 0;
    let mut pos = start;
    let track_iter = std::iter::from_fn(|| {
        if lap_count > 9 {
            return None;
        }
        if grid.get(pos.step(dir)).is_none() {
            dir = dir.turn_left();
        }
        pos = pos.step(dir);
        let here = *grid.get(pos).unwrap();
        if here == 'S' {
            lap_count += 1;
        }
        Some(here)
    }).collect::<Vec<char>>();

    let foo = input.lines().map(|l| {
        let (name, instructions) = l.split_once(":").unwrap();
        let instructions = instructions.split(",").map(|x| x.chars().next().unwrap()).cycle();
        let gathered = run_race(instructions, track_iter.iter().cloned());
        (name, gathered)
    }).sorted_by(|a,b| b.1.cmp(&a.1)).collect::<Vec<_>>();
    foo.into_iter().map(|x| x.0).collect()

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
        if lap_count > 2023 {
            return None;
        }
        let pos_ok = |p| if let Some(x) = grid.get(p) {
             *x != ' ' 
        } else { false };
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
    }).collect::<Vec<char>>();
    let str : String = track_iter.iter().collect();
    println!("{}", &str[0..300]);
    let s_count = track_iter.iter().filter(|x| **x == 'S').count();
    assert_eq!(s_count,2024);
    dbg!(track_iter.len() / 2024);
    let rival_score = {
        let (_name, instructions) = input.split_once(":").unwrap();
        let instructions = instructions.split(",").map(|x| x.chars().next().unwrap()).cycle();
        run_race(instructions, track_iter.iter().cloned())
    };
    println!("Rival score is {rival_score}");

    let mut beat_count = 0;
    let mut fail_count = 0;
    for plan in "+++++---===".chars().permutations(11).unique() {
        let score = run_race(plan.iter().cloned().cycle(), track_iter.iter().cloned());
        if score > rival_score {
            beat_count += 1;
        } else {
            fail_count += 1;
        }

    }
    dbg!(fail_count);

    beat_count
}
fn run_race<A, B> (mut plan: A, track: B) -> u64 
where A : Iterator<Item = char>,
      B : Iterator<Item = char>
{
    let mut gathered = 0;
    let mut power = 10u64;
    for c in track {
        match (c,plan.next().unwrap()) {
            ('-',_) | ('=', '-') | ('S', '-') => power = power.saturating_sub(1),
            ('+',_) | ('=', '+') | ('S', '+') => power += 1,
            _ => (),
        }
        gathered += power;
    }
    gathered
}


#[cfg(test)]
mod test {
    use super::*;
    const EG1 : &str = "A:+,-,=,=
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