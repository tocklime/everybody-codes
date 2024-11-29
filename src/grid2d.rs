use std::{
    convert::Into,
    fmt::{Debug, Display, Write},
    iter,
    ops::{Index, IndexMut},
};

use itertools::Itertools;
use num::Integer;

use crate::{aabb::Aabb, cartesian::Point, nums::add_i_mod};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid2d<T> {
    data: Vec<T>,
    size: Coord,
}

pub type Coord = Point<usize>;
pub type ICoord = Point<isize>;
impl<T: Copy> Grid2d<T> {
    pub fn from_elem<TC: Into<Coord>>(size: TC, elem: T) -> Self {
        let size = size.into();
        Self {
            data: vec![elem; size.y * size.x],
            size,
        }
    }
}
impl<T> Grid2d<T> {
    pub fn from_fn<F, TC: Into<Coord>>(size: TC, mut f: F) -> Self
    where
        F: FnMut(Coord) -> T,
    {
        let size = size.into();
        let mut data = Vec::with_capacity(size.y * size.x);
        for a in 0..size.y {
            for b in 0..size.x {
                data.push(f(Point::new(b, a)));
            }
        }
        Self { data, size }
    }
}
impl<T, TC> Index<TC> for Grid2d<T>
where
    TC: Into<Coord>,
{
    type Output = T;

    fn index(&self, index: TC) -> &Self::Output {
        let index = index.into();
        &self.data[index.y * self.size.x + index.x]
    }
}

impl<T, TC: Into<Coord>> IndexMut<TC> for Grid2d<T> {
    fn index_mut(&mut self, index: TC) -> &mut Self::Output {
        let index = index.into();
        &mut self.data[index.y * self.size.x + index.x]
    }
}
impl<T> Index<usize> for Grid2d<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for Grid2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl<T: Display> Display for Grid2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (p, t) in self.indexed_iter() {
            f.write_fmt(format_args!("{t}"))?;
            if p.x == self.size.x - 1 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}
impl<T: Copy> Grid2d<T> {
    pub fn grow_and_invalidate_all_data(&mut self, new_size: Coord, new_t: T) {
        self.size = new_size;
        let need_len = self.size.y * self.size.x;
        self.data.extend((self.data.len()..need_len).map(|_| new_t));
    }
}
impl<T: PartialEq<T>> Grid2d<T> {
    pub fn insert<TC: Into<Coord>>(&mut self, p: TC, val: T) -> bool {
        let p = p.into();
        let x = &mut self[p];
        if *x != val {
            *x = val;
            true
        } else {
            false
        }
    }
}
impl<T: Copy> Grid2d<T> {
    pub fn sub_grid_copied<TC1, TC2>(&self, base: TC1, size: TC2) -> Grid2d<T>
    where
        TC1: Into<Coord>,
        TC2: Into<Coord>,
    {
        let base = base.into();
        Grid2d::from_fn(size, |x| self[base + x])
    }
}
impl<T> Grid2d<T> {
    pub fn sub_grid<TC1, TC2>(&self, base: TC1, size: TC2) -> Grid2d<&T>
    where
        TC1: Into<Coord>,
        TC2: Into<Coord>,
    {
        let base = base.into();
        Grid2d::from_fn(size, |x| &self[base + x])
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[must_use]
    pub fn get<TC: Into<Coord>>(&self, p: TC) -> Option<&T> {
        let p = p.into();
        if p.y < self.size.y && p.x < self.size.x {
            Some(&self[p])
        } else {
            None
        }
    }
    #[must_use]
    pub fn get_mut<TC: Into<Coord>>(&mut self, p: TC) -> Option<&mut T> {
        let p = p.into();
        if p.y < self.size.y && p.x < self.size.x {
            Some(&mut self[p])
        } else {
            None
        }
    }
    #[must_use]
    pub fn to_u(p: ICoord) -> Option<Coord> {
        Some((p.y.try_into().ok()?, p.x.try_into().ok()?).into())
    }
    #[must_use]
    pub fn get_i<TC: Into<ICoord>>(&self, p: TC) -> Option<&T> {
        let p = p.into();
        Self::to_u(p).and_then(|p| self.get(p))
    }

    pub fn dim(&self) -> Coord {
        self.size
    }
    pub fn indexes(&'_ self) -> impl Iterator<Item = Coord> {
        let max = self.size;
        (0..max.y).cartesian_product(0..max.x).map(Into::into)
    }
    pub fn indexed_iter(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(x, v)| (x.div_mod_floor(&self.size.x).into(), v))
    }
    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (Coord, &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(x, v)| (x.div_mod_floor(&self.size.x).into(), v))
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }
    #[must_use]
    pub fn ordered_neighbours_with_self(&self, p: Coord) -> [Option<Coord>; 9] {
        let s = self.dim();
        let up = p.y.checked_sub(1);
        let left = p.x.checked_sub(1);
        let down = if p.y + 1 == s.y { None } else { Some(p.y + 1) };
        let right = if p.x + 1 == s.x { None } else { Some(p.x + 1) };
        [
            up.and_then(|y| left.map(|x| (y, x))),
            up.map(|y| (y, p.x)),
            up.and_then(|y| right.map(|x| (y, x))),
            left.map(|x| (p.y, x)),
            Some(p.into()),
            right.map(|x| (p.y, x)),
            down.and_then(|y| left.map(|x| (y, x))),
            down.map(|y| (y, p.x)),
            down.and_then(|y| right.map(|x| (y, x))),
        ]
        .map(|x| x.map(Into::into))
    }
    pub fn optional_neighbours(&'_ self, p: Coord) -> impl Iterator<Item = Option<Coord>> {
        let s = self.dim();
        [
            (p.y.wrapping_sub(1), p.x.wrapping_sub(1)), //UL
            (p.y.wrapping_sub(1), p.x),                 //Up
            (p.y.wrapping_sub(1), p.x + 1),             //UR
            (p.y, p.x.wrapping_sub(1)),                 //Left
            (p.y, p.x + 1),                             //Right
            (p.y + 1, p.x.wrapping_sub(1)),             //DL
            (p.y + 1, p.x),                             //Down
            (p.y + 1, p.x + 1),                         //DR
        ]
        .map(Into::into)
        .into_iter()
        .map(move |x: Coord| (x.y < s.y && x.x < s.x).then_some(x))
    }
    pub fn neighbours_with_diagonals(&'_ self, p: Coord) -> impl Iterator<Item = Coord> {
        let s = self.dim();
        [
            (p.y.wrapping_sub(1), p.x),                 //Up
            (p.y.wrapping_sub(1), p.x + 1),             //UR
            (p.y, p.x + 1),                             //Right
            (p.y + 1, p.x + 1),                         //DR
            (p.y + 1, p.x),                             //Down
            (p.y + 1, p.x.wrapping_sub(1)),             //DL
            (p.y, p.x.wrapping_sub(1)),                 //Left
            (p.y.wrapping_sub(1), p.x.wrapping_sub(1)), //UL
        ]
        .map(Into::into)
        .into_iter()
        .filter(move |x: &Coord| x.y < s.y && x.x < s.x)
    }
    pub fn neighbours_array_ordered(&'_ self, p: Coord) -> [Option<Coord>; 4] {
        let s = self.dim();
        [
            p.y.checked_sub(1).map(|x| (x, p.x)),
            p.x.checked_sub(1).map(|x| (p.y, x)),
            (p.y + 1 < s.y).then_some((p.y + 1, p.x)),
            (p.x + 1 < s.x).then_some((p.y, p.x + 1)),
        ]
        .map(|x| x.map(Into::into))
    }
    pub fn neighbours(&'_ self, p: Coord) -> impl Iterator<Item = Coord> {
        let s = self.dim();
        [
            (p.y.wrapping_sub(1), p.x),
            (p.y, p.x.wrapping_sub(1)),
            (p.y + 1, p.x),
            (p.y, p.x + 1),
        ]
        .map(Into::into)
        .into_iter()
        .filter(move |x: &Coord| x.y < s.y && x.x < s.x)
    }
    pub fn is_edge(&self, p: Coord) -> bool {
        p.x == 0 || p.y == 0 || p.x == self.size.x - 1 || p.y == self.size.y - 1
    }

    /// Returns all values in the grid by taking steps of `relative` from `start`.
    /// Includes the value at `start`.
    pub fn values_in_direction<T1: Into<Coord>, T2: Into<ICoord>>(
        &self,
        start: T1,
        relative: T2,
    ) -> impl Iterator<Item = (Coord, &T)> {
        let start = start.into();
        let relative = relative.into();
        let mut pos: ICoord = (start.y as isize, start.x as isize).into();
        iter::from_fn(move || {
            let here = Self::to_u(pos)?;
            pos.y += relative.y;
            pos.x += relative.x;
            let next = self.get(here);
            next.map(|p| (here, p))
        })
    }
    #[must_use]
    pub fn get_row(&self, y: usize) -> &[T] {
        let w = self.size.x;
        &self.data[y * w..(y + 1) * w]
    }

    pub fn get_col(&self, x: usize) -> impl Iterator<Item = &T> {
        let w = self.size.x;
        self.data.iter().skip(x).step_by(w)
    }
    #[must_use]
    pub fn relative_lookup(&self, p: Coord, relative: ICoord) -> Option<&T> {
        self.get(p + relative)
    }
    #[must_use]
    pub fn wraparound_relative_lookup<TU: Into<Coord>, TI: Into<ICoord>>(
        &self,
        p: TU,
        relative: TI,
    ) -> &T {
        let p = p.into();
        let relative = relative.into();
        let d = self.dim();
        let y = add_i_mod(p.y, &relative.y, d.y);
        let x = add_i_mod(p.x, &relative.x, d.x);
        &self[Point { y, x }]
    }
    pub fn wraparound_neighbours(&self, Point { y, x }: Coord) -> [Coord; 4] {
        let s = self.dim();
        [
            ((y + s.y - 1) % s.y, x),
            (y, (x + s.x - 1) % s.x),
            (y, (x + 1) % s.x),
            ((y + 1) % s.y, x),
        ]
        .map(Into::into)
    }
    pub fn to_string_with<F>(&self, disp: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut ans = String::with_capacity(self.data.len());
        for (p, t) in self.indexed_iter() {
            ans.push_str(&disp(t));
            if p.x == self.size.x - 1 {
                ans.push('\n');
            }
        }
        ans
    }
    pub fn find_bb<F>(&self, pred: F) -> Aabb<usize>
    where
        F: Fn(&T) -> bool,
    {
        self.indexed_iter()
            .filter(|&(_, x)| pred(x))
            .map(|(p, _)| p)
            .collect()
    }
    pub fn render_section_with<F>(&self, bb: Aabb<usize>, disp: F) -> String
    where
        F: Fn(&T) -> String,
    {
        let mut ans = String::with_capacity(bb.area());
        for y in bb.bottom_left.y..=bb.top_right.y {
            for x in bb.bottom_left.x..=bb.top_right.x {
                ans.push_str(&disp(&self[Point { y, x }]));
            }
            ans.push('\n');
        }
        ans
    }
    pub fn from_iter<C, F>(input: impl Iterator<Item = C>, conv: F, new_row_marker: C) -> Self
    where
        C: Eq,
        F: Fn(C) -> T,
        T: Debug,
    {
        let mut stride = None;
        let mut data = Vec::with_capacity(input.size_hint().0);
        // let mut all = Vec::new();
        let mut rows = 0;
        let mut this_row_len = 0;
        for c in input {
            // all.push(c);
            if c == new_row_marker {
                rows += 1;
                if let Some(other_rows) = stride {
                    // dbg!(&all, &data, &stride, rows, this_row_len);
                    assert_eq!(
                        this_row_len, other_rows,
                        "Unequal line lengths: {other_rows} and {this_row_len}"
                    );
                } else {
                    stride = Some(this_row_len);
                }
                this_row_len = 0;
            } else {
                this_row_len += 1;
                data.push(conv(c));
            }
        }
        Self {
            data,
            size: (rows, stride.unwrap()).into(),
        }
    }
    pub fn from_str<F>(input: &str, conv: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let mut stride = None;
        let mut data = Vec::with_capacity(input.len());
        let mut rows = 0;
        for l in input.lines() {
            rows += 1;
            match (stride, l.len()) {
                (None, l) => stride = Some(l),
                (Some(a), b) if a != b => panic!("Not equal line lengths"),
                _ => {}
            }
            for c in l.chars() {
                data.push(conv(c));
            }
        }
        Self {
            data,
            size: (rows, stride.unwrap()).into(),
        }
    }
    pub fn from_str_with_index<F>(input: &str, mut conv: F) -> Self
    where
        F: FnMut(Coord, char) -> T,
    {
        let mut stride = None;
        let mut data = Vec::with_capacity(input.len());
        let mut rows = 0;
        for (row, l) in input.lines().enumerate() {
            match (stride, l.len()) {
                (None, l) => stride = Some(l),
                (Some(a), b) if a != b => panic!("Not equal line lengths"),
                _ => {}
            }
            for (col, c) in l.chars().enumerate() {
                data.push(conv((row, col).into(), c));
            }
            rows += 1;
        }
        Self {
            data,
            size: (rows, stride.unwrap()).into(),
        }
    }
    pub fn map<F, TO>(&self, mut f: F) -> Grid2d<TO>
    where
        F: FnMut(Coord, &T) -> TO,
    {
        Grid2d::from_fn(self.dim(), |p| f(p, &self[p]))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn values_in_direction_test() {
        const GRID: &str = "012
345
678";
        let g = Grid2d::from_str(GRID, |c| c);
        assert_eq!(
            g.values_in_direction((1, 1), (1, 0))
                .map(|x| *x.1)
                .collect_vec(),
            ['4', '7']
        );
        assert_eq!(
            g.values_in_direction((1, 1), (-1, 0))
                .map(|x| *x.1)
                .collect_vec(),
            ['4', '1']
        );
        assert_eq!(
            g.values_in_direction((2, 0), (-1, 0))
                .map(|x| *x.1)
                .collect_vec(),
            ['6', '3', '0']
        );
        assert_eq!(
            g.values_in_direction((0, 2), (-1, 0))
                .map(|x| *x.1)
                .collect_vec(),
            ['2']
        );
        assert_eq!(
            g.values_in_direction((0, 2), (0, -1))
                .map(|x| *x.1)
                .collect_vec(),
            ['2', '1', '0']
        );
        assert_eq!(
            g.values_in_direction((0, 0), (0, 1))
                .map(|x| *x.1)
                .collect_vec(),
            ['0', '1', '2']
        );
    }
}
