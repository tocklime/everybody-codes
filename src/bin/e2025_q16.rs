use everybody_codes::nums::bin_search;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q16_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q16_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q16_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn blocks_to_spell(ns: &[usize]) -> Vec<usize> {
    let mut ans = Vec::new();
    for (&count, pos) in ns.iter().zip(1..) {
        let total_here_from_prev = ans.iter().filter(|&&n| pos % n == 0).count();
        let remaining = count - total_here_from_prev;
        if remaining > 0 {
            ans.push(pos);
        }
    }
    ans
}
fn blocks_in_wall_of_len(spell: &[usize], len: usize) -> usize {
    spell
        .iter()
        .map(|s| {
            //between 1 and len, how many numbers are there that are 0 mod n?
            len / s
        })
        .sum()
}
fn solve<const PART: usize>(input: &str) -> usize {
    let ns: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    match PART {
        1 => blocks_in_wall_of_len(&ns, 90),
        2 => blocks_to_spell(&ns).into_iter().product(),
        3 => {
            let total_blocks = 202520252025000usize;
            let spell = blocks_to_spell(&ns);
            let cycle_len = spell.iter().copied().fold(1, num::integer::lcm);
            let total_per_cycle = blocks_in_wall_of_len(&spell, cycle_len);
            let complete_cycles = total_blocks / total_per_cycle;
            let blocks_left = total_blocks % total_per_cycle;
            let last_complete_col = bin_search(
                &|n| blocks_in_wall_of_len(&spell, n),
                blocks_left,
                cycle_len,
                0,
            );
            complete_cycles * cycle_len + last_complete_col
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "1,2,3,5,9";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 193);
    }
    #[test]
    fn p2_example() {
        assert_eq!(
            solve::<2>("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2"),
            270
        );
    }
    #[test]
    fn p3_example() {
        assert_eq!(
            solve::<3>("1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2"),
            94439495762954
        );
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 243);
        assert_eq!(solve::<2>(P2_INPUT), 125655957504);
        assert_eq!(solve::<3>(P3_INPUT), 98385663668319);
    }
}
