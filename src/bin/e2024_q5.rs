use std::collections::{HashMap, HashSet};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q05_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q05_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q05_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let mut d = Dance::new(input);
    match PART {
        1 =>  {    
            for _ in 0..10 {
                d.do_round();
            }
            d.shout()
        }
        2 => {
            let mut nums : HashMap<usize,usize> = HashMap::new();
            for round in 1.. {
                d.do_round();
                let x = nums.entry(d.shout()).or_default();
                *x += 1;
                if *x == 2024 {
                    return round * d.shout()
                }
            }

            0
        }
        3 => {
            let mut seen = HashSet::new();
            let mut max_heard = 0;
            for _ in 1.. {
                d.do_round();
                let shout = d.shout();
                max_heard = max_heard.max(shout);
                if seen.contains(&d) {
                    return max_heard;
                }
                seen.insert(d.clone());
            }
            0
        }
        _ => unimplemented!()
    }
}

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
struct Dance {
    columns: Vec<Vec<usize>>,
    next_column_to_dance: usize
}

fn bounding_power_of_ten(mut n: usize) -> usize {
    let mut ans = 10;
    while n > 9  {
        ans *=10;
        n /= 10;
    }
    ans
}

impl Dance {
    pub fn new(config: &str) -> Self {
        let mut columns = Vec::new();
        for l in config.lines() {
            for (ix, p) in l.split(" ").enumerate() {
                while columns.len() < ix + 1 {
                    columns.push(Vec::new());
                }
                columns[ix].push(p.parse().unwrap())
            }
        }
        Self { columns, next_column_to_dance: 0 }
    }
    #[allow(dead_code)]
    pub fn draw(&self) {
        let max_ix = self.columns.iter().map(|c| c.len()).max().unwrap();
        for row in 0..max_ix {
            for c in self.columns.iter() {
                if let Some(p) = c.get(row) {
                    print!(" {p}");
                } else {
                    print!("  ");
                }
            }
            println!();
        }
        println!();
    }
    pub fn shout(&self) -> usize {
        self.columns.iter().fold(0, |a,e| (bounding_power_of_ten(e[0]))*a+e[0])
    }
    pub fn do_round(&mut self) {
        let person = self.columns[self.next_column_to_dance].remove(0);
        let new_column = (self.next_column_to_dance + 1) % self.columns.len();
        let next_col = &mut self.columns[new_column];
        let mut insertion_point = (person-1) % (next_col.len() * 2);
        if insertion_point > next_col.len()  {
            insertion_point = (2 * next_col.len()) - insertion_point;
        }
        next_col.insert(insertion_point, person);
        self.next_column_to_dance = new_column;
    }
}

#[cfg(test)]
mod test {
    use std::iter;

    use super::*;
    const EG1 : &str = "2 3 4 5\n3 4 5 2\n4 5 2 3\n5 2 3 4";
    const EG2 : &str = "2 3 4 5\n6 7 8 9";
    const CORRECT_SHOUTS : [usize; 10] = [ 3345, 3245, 3255, 3252, 4252, 4452, 4422, 4423, 2423, 2323];
    #[test]
    fn p1_example() {
        let mut d = Dance::new(EG1);
        let our_shouts = iter::from_fn(|| {
            d.do_round();
            Some(d.shout())}
        ).take(CORRECT_SHOUTS.len());
        assert!(our_shouts.eq(CORRECT_SHOUTS.iter().cloned()));
        assert_eq!(solve::<1>(EG1), 2323);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>(EG2), 50877075);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG2), 6584);
    }
}