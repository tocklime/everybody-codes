use everybody_codes::{cartesian::Point, grid2d::Grid2d};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q12_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q12_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q12_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve3(P3_INPUT));
}
fn set_fire(g: &Grid2d<u8>, starts: Vec<Point<usize>>, alight: &mut Grid2d<bool>) {
    let mut fringe: Vec<Point<usize>> = starts.clone();
    fringe.iter().for_each(|p| alight[*p] = true);
    while let Some(x) = fringe.pop() {
        for n in g.neighbours(x) {
            if g[n] <= g[x] && !alight[n] {
                alight[n] = true;
                fringe.push(n);
            }
        }
    }
}
fn find_best(g: &Grid2d<u8>, already_alight: &Grid2d<bool>) -> (Point<usize>, Grid2d<bool>) {
    let (_count, alight, point) = g
        .indexes()
        .filter(|&p| { 
            //only consider those that are bigger than all their (unlit) neighbours.
            g.neighbours(p).all(|x| already_alight[x] || g[x] <= g[p])
        })
        .map(|x| {
            let mut alight = already_alight.clone();
            set_fire(g, vec![x], &mut alight);
            let count = alight.iter().filter(|x| **x).count();
            (count, alight, x)
        })
        .max_by_key(|z| z.0)
        .unwrap();
    (point, alight)
}
fn solve3(input: &str) -> usize {
    let g: Grid2d<u8> = Grid2d::from_str(input, |x| (x as u8) - b'0');
    let alight = Grid2d::from_elem(g.dim(), false);
    (0..3)
        .fold(alight, |alight, _| find_best(&g, &alight).1)
        .iter()
        .filter(|x| **x)
        .count()
}

fn solve<const PART: usize>(input: &str) -> usize {
    let g: Grid2d<u8> = Grid2d::from_str(input, |x| (x as u8) - b'0');
    let fringe = match PART {
        1 => vec![Point::new(0, 0)],
        2 => vec![Point::new(0, 0), g.dim() - Point::new(1, 1)],
        _ => unimplemented!(),
    };
    let mut alight = Grid2d::from_elem(g.dim(), false);
    set_fire(&g, fringe, &mut alight);
    alight.iter().filter(|x| **x).count()
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "989611
857782
746543
766789";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 16);
    }
    #[test]
    fn p2_example() {
        assert_eq!(
            solve::<2>(
                "9589233445
9679121695
8469121876
8352919876
7342914327
7234193437
6789193538
6781219648
5691219769
5443329859"
            ),
            58
        );
    }
    #[test]
    fn p3_example() {
        const EG1: &str = "5411
3362
5235
3112";
        assert_eq!(solve3(EG1), 14);
        const EG2: &str = "41951111131882511179
32112222211508122215
31223333322105122219
31234444432147511128
91223333322176021892
60112222211166431583
04661111166111111746
01111119042122222177
41222108881233333219
71222127839122222196
56111026279711111507";
        assert_eq!(solve3(EG2), 133);
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 251);
        assert_eq!(solve::<2>(P2_INPUT), 5640);
        assert_eq!(solve3(P3_INPUT), 4208);
    }
}
