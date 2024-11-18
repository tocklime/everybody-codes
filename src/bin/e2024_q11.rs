use std::collections::HashMap;

use itertools::Itertools;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q11_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q11_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q11_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let map : HashMap<&str,Vec<&str>>= input.lines().map(|l| {
        let (key,res) = l.split_once(':').unwrap();
        let res : Vec<&str> = res.split(',').collect();
        (key,res)
    }).collect();


    if PART < 3 {
        let (days,start) = match PART {
            1 => (4,"A"),
            2 => (10,"Z"),
            _ => unimplemented!()
            
        };
        let mut termites = vec![start];
        for _d in 0..days {
            termites = termites.into_iter().flat_map(|x| &map[&x]).cloned().collect();
        }
        termites.len()
    } else {
        let mut dp = HashMap::new();
        for x in 0..20 {
            println!("Doing {x} days left");
            for k in map.keys() {
                recur(&mut dp, &map, &k, x);
            }
            println!("Done. dp now has {} entries", dp.len());
        }

        let pops :Vec<usize> = map.keys().map(|_k| {
            println!("Trying {_k}");
            recur(&mut dp, &map, _k, 20)
            // let mut termites : Vec<&str> = vec![*k];
            // for _d in 0..20 {
            //     termites = termites.into_iter().flat_map(|x| &map[x]).map(|&x| x).collect();
            // }
            // termites.len()
        }).sorted().collect();
        pops[pops.len()-1] - pops[0]
    }
}
fn recur<'a>(dp: &mut HashMap<(&'a str, usize), usize>, map : &HashMap<&'a str,Vec<&'a str>>, termite: &'a str, day_count: usize) -> usize {
    if let Some(x) = dp.get(&(termite,day_count)) {
        return *x
    }
    if day_count == 0 {
        return 1;
    }
    let mut ans = 0;
    for &x in &map[&termite] {
        ans += recur(dp, map, x, day_count-1)
    }
    dp.insert((termite,day_count), ans);
    ans
}


#[cfg(test)]
mod test {
    use super::*;
    const EG1 : &str = "A:B,C
B:C,A
C:A";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 8);
    }
    const EG3 : &str = "A:B,C
B:C,A,A
C:A";
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG3), 268815);
    }
}