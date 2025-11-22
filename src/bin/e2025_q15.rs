use std::collections::{BTreeMap, BTreeSet};

use everybody_codes::cartesian::{Dir, Point};
use itertools::Itertools;
use tiny_skia::{Color, Paint, Stroke, Transform};

const P1_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q15_p1.txt");
const P2_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q15_p2.txt");
const P3_INPUT: &str = include_str!("../../inputs/everybody_codes_e2025_q15_p3.txt");

fn main() {
    println!("P1: {}", solve::<1>(P1_INPUT));
    println!("P2: {}", solve::<2>(P2_INPUT));
    println!("P3: {}", solve::<3>(P3_INPUT));
}

#[allow(dead_code)]
fn draw(input: &str) -> usize {
    //0 find min and max point.
    let mut dir = Dir::Up;
    let mut point = Point::new(0i64, 0);
    let mut min: Point<i64> = Point::default();
    let mut max: Point<i64> = Point::default();
    let mut walls = Vec::new();
    for s in input.split(',') {
        match &s[0..1] {
            "L" => dir = dir.turn_left(),
            "R" => dir = dir.turn_right(),
            _ => panic!("{s}"),
        }
        let count: i64 = s[1..].parse().unwrap();
        let new_point = point + (dir * count);
        walls.push((point, new_point));
        point = new_point;
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
    }
    let size = Point::new(max.x - min.x, max.y - min.y);
    let scale = 50000;
    // dbg!(size);
    let mut im = tiny_skia::Pixmap::new((size.x / scale) as u32, (size.y / scale) as u32).unwrap();
    let mut pb = tiny_skia::PathBuilder::new();
    pb.move_to((-min.x / scale) as f32, (-min.y / scale) as f32);
    for w in &walls {
        pb.line_to(
            ((w.1.x - min.x) / scale) as f32,
            ((w.1.y - min.y) / scale) as f32,
        );
    }
    let p = pb.finish().unwrap();
    let stroke = Stroke::default();
    let paint = Paint {
        shader: tiny_skia::Shader::SolidColor(Color::WHITE),
        ..Default::default()
    };
    let tr = Transform::default();
    im.stroke_path(&p, &paint, &stroke, tr, None);
    im.save_png("path.png").unwrap();
    0
    // let important_corners = [
    //     1,4,7,13,17,25,31,

    // ];
}
fn solve<const PART: usize>(input: &str) -> usize {
    //0 find min and max point.
    let mut dir = Dir::Up;
    let mut point = Point::new(0i64, 0);
    let mut min: Point<i64> = Point::default();
    let mut max: Point<i64> = Point::default();
    let mut walls = Vec::new();
    let mut x_walls: BTreeMap<i64, BTreeSet<(i64, i64)>> = BTreeMap::new();
    let mut y_walls: BTreeMap<i64, BTreeSet<(i64, i64)>> = BTreeMap::new();
    let mut interesting_x = BTreeSet::new();
    let mut interesting_y = BTreeSet::new();
    interesting_x.insert(0);
    interesting_y.insert(0);
    for s in input.split(',') {
        match &s[0..1] {
            "L" => dir = dir.turn_left(),
            "R" => dir = dir.turn_right(),
            _ => panic!("{s}"),
        }
        let mut count: i64 = s[1..].parse().unwrap();
        if point == Point::default() {
            point = point.step(dir);
            count -= 1;
        }
        let new_point = point + (dir * (count));
        let p_sm = point.min(new_point);
        let p_bi = point.max(new_point);
        walls.push((p_sm, p_bi));
        if point.x == new_point.x {
            //vert_wall.
            let sm = point.y.min(new_point.y);
            let bi = point.y.max(new_point.y);
            x_walls.entry(point.x).or_default().insert((sm, bi));
        } else {
            let sm = point.x.min(new_point.x);
            let bi = point.x.max(new_point.x);
            y_walls.entry(point.y).or_default().insert((sm, bi));
        }
        point = new_point;
        interesting_x.insert(point.x + 1);
        interesting_x.insert(point.x - 1);
        interesting_y.insert(point.y + 1);
        interesting_y.insert(point.y - 1);

        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
        // println!("Done {s}");
    }
    interesting_x.insert(point.x);
    interesting_y.insert(point.y);

    if let Some(w) = walls.last_mut() {
        if w.0 == point {
            if w.0.x == w.1.x {
                //vert, and this is the smaller number.
                w.0.y += 1;
            } else {
                w.0.x += 1;
            }
        } else if w.0.x == w.1.x {
            w.1.y -= 1;
        } else {
            w.1.x -= 1;
        }
    }

    let start = Point::new(0, 0);
    let end = point;

    let collides = |a: Point<i64>, b: Point<i64>| {
        let sm = a.min(b);
        let bi = a.max(b);
        walls.iter().any(|(w_a, w_b)| {
            let no_collide = sm.x > w_b.x || bi.x < w_a.x || sm.y > w_b.y || bi.y < w_a.y;
            // if !no_collide {
            // println!("line {sm}->{bi} collides with wall {w_a}->{w_b}");
            // }
            !no_collide
        })
    };
    // // println!("xs: {interesting_x:?}");
    // // println!("ys: {interesting_y:?}");
    // println!("walls: {walls:?}");

    let path = pathfinding::directed::bfs::bfs(
        &start,
        |p| {
            let neighbours = [
                //up
                interesting_y
                    .range(..p.y)
                    .next_back()
                    .map(|&y| Point::new(p.x, y)),
                //down
                interesting_y
                    .range(p.y + 1..)
                    .next()
                    .map(|&y| Point::new(p.x, y)),
                //left
                interesting_x
                    .range(..p.x)
                    .next_back()
                    .map(|&x| Point::new(x, p.y)),
                //right
                interesting_x
                    .range(p.x + 1..)
                    .next()
                    .map(|&x| Point::new(x, p.y)),
            ];
            neighbours
                .into_iter()
                .flatten()
                .filter(|n| !collides(*p, *n))
                .collect::<Vec<_>>()
        },
        |p| p == &end,
    )
    .unwrap();
    path.iter()
        .tuple_windows()
        .map(|(a, b)| {
            let x_diff = (a.x - b.x).abs();
            let y_diff = (a.y - b.y).abs();
            x_diff + y_diff
        })
        .sum::<i64>() as usize
}

#[cfg(test)]
mod test {
    use super::*;
    const EG1: &str = "L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3";
    #[test]
    fn p1_example() {
        assert_eq!(solve::<1>("R3,R4,L3,L4,R3,R6,R9"), 6);
        assert_eq!(solve::<1>(EG1), 16);
        // assert_eq!(solve_and_draw_together(EG1), 16);
    }
    // #[test]
    // fn p2_example() {
    //     assert_eq!(solve::<2>(EG2), 0);
    // }
    // #[test]
    // fn p3_example() {
    //     assert_eq!(solve::<3>(EG3), 0);
    // }

    // #[test]
    // fn correct_answers() {
    //     assert_eq!(solve::<1>(P1_INPUT), 0);
    //     assert_eq!(solve::<2>(P2_INPUT), 0);
    //     assert_eq!(solve::<3>(P3_INPUT), 0);
    // }
}

//26500835: inc len, inc 1st.
