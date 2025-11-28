use everybody_codes::inputs::parse_input_from_str_sep_by;

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q19_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q19_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q19_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}
fn solve<const PART: usize>(input: &str) -> usize {
    let start = (0, 0, false); //x, height, next column.
    let columns: Vec<Vec<i32>> = input
        .lines()
        .map(|x| parse_input_from_str_sep_by::<i32>(x, ","))
        .collect();
    let path = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |&(x, y, _)| {
            // println!("Doing {x} {y}");
            let Some(next_wall_ix) = columns.iter().position(|w| w[0] > x) else {
                return vec![((x,y, true),0)]
            };
            // let at_wall = columns[next_wall_ix][0] == x;
            let next_wall_x = columns[next_wall_ix][0];
            let next_next_wall_ix = columns.iter().position(|w| w[0] > next_wall_x);
            let next_walls = if let Some(ix) = next_next_wall_ix {
                &columns[next_wall_ix..ix]
            } else {
                &columns[next_wall_ix..]
            };
            if next_walls.len() == 0 {
                return vec![((x+1,y-1, false),0)]
            }
            let dist_to_next_wall = next_walls[0][0] - x;
            //we're going to teleport to a gap.
            let min_y = (y-dist_to_next_wall).max(0);
            (min_y..=y+dist_to_next_wall).filter(|&target_y| {
                target_y % 2 == next_wall_x % 2 && //check we're on the allowed checker-board pattern.
                next_walls.iter().any(|w| target_y >= w[1] && target_y < w[1] + w[2]) //check we haven't crashed.
            }).map(|new_y| {
                let min_y = y-dist_to_next_wall;
                let flight_cost = (new_y - min_y)/2;
                ((next_walls[0][0], new_y, false),flight_cost as usize)
            }).collect::<Vec<_>>()
        },
        |(_, _, done)| *done
    );
    path.unwrap().1
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "7,7,2
12,0,4
15,5,3
24,1,6
28,5,5
40,8,2";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>(EG1), 24);
    }
    #[test]
    fn p2_example() {
        assert_eq!(
            solve::<2>(
                "7,7,2
7,1,3
12,0,4
15,5,3
24,1,6
28,5,5
40,3,3
40,8,2"
            ),
            22
        );
    }

    #[test]
    fn correct_answers() {
        assert_eq!(solve::<1>(P1_INPUT), 56);
        assert_eq!(solve::<2>(P2_INPUT), 714);
        assert_eq!(solve::<3>(P3_INPUT), 4472779);
    }
}

//todo: this is really slow because I'm enumerating every possible y value we could use to fly through each gap.
//would be better to dynamic programming it, probably. "best way to get to this (x,y) is with cost X". Prune out the impossibilities.
//keep stepping along.