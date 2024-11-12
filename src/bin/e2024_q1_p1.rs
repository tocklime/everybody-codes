use itertools::Itertools;

const P1_INPUT : &str = include_str!("../../inputs/everybody_codes_e2024_q01_p1.txt");
const P2_INPUT : &str = include_str!("../../inputs/everybody_codes_e2024_q01_p2.txt");
const P3_INPUT : &str = include_str!("../../inputs/everybody_codes_e2024_q01_p3.txt");

fn potions_required(c: char) -> Option<u32> {
    match c {
        'A' => Some(0),
        'B' => Some(1),
        'C' => Some(3),
        'D' => Some(5),
        'x' => None,
        _ => panic!("Unknown char {c}")
    }
}
fn solve<const GROUP_SIZE: usize>(input: &str) -> u32 {
    input.chars().chunks(GROUP_SIZE).into_iter().map(|x| {
        let (total, count) = x.map(|i| {
            match potions_required(i) {
                Some(c) => (c,1),
                None => (0,0),
            }
        }).fold((0,0), |a,e| (a.0+e.0,a.1+e.1));
        total + (count - 1) * count
    }).sum()
}
fn main() {
    println!("P1: {}",solve::<1>(P1_INPUT));
    println!("P2: {}",solve::<2>(P2_INPUT));
    println!("P3: {}",solve::<3>(P3_INPUT));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>("ABBAC"), 5);
    }
    #[test]
    fn p2_example() {
        assert_eq!(solve::<2>("AxBCDDCAxD"),28);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>("xBxAAABCDxCC"),30);
    }
}