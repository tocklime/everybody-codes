
const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q04_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q04_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2024_q04_p3.txt");
fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
   let mut ns : Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
   let target = if PART == 3 {
        ns.sort();
        ns[ns.len()/2]
   } else {
        *ns.iter().min().unwrap()
   };
   ns.iter().map(|n| n.abs_diff(target)).sum()
}


#[cfg(test)]
mod test {
    use super::*;
    const EG1 : &str = "3\n4\n7\n8";
    const EG3 : &str = "2\n4\n5\n6\n8";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 10);
    }
    #[test]
    fn p3_example() {
        assert_eq!(solve::<3>(EG3), 8);
    }
}