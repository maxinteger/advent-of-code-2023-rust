use crate::utils::coords::Coord;
use itertools::Itertools;
use std::fmt::Debug;
use std::fmt::Display;
use std::io::Write;

pub struct AsciiMatrix<T> {
    pub cols: usize,
    pub rows: usize,
    pub data: Vec<T>,
}

type Neighbor<T> = (T, Coord);

impl<T: Clone + Display + Default> AsciiMatrix<T> {
    pub fn new(cols: usize, rows: usize, init: T) -> AsciiMatrix<T> {
        AsciiMatrix {
            cols,
            rows,
            data: vec![init; cols * rows],
        }
    }

    pub fn from_vec_of_vec(data: Vec<Vec<T>>) -> AsciiMatrix<T> {
        let rows = data.len();
        let cols = data[0].len();
        let mut matrix = AsciiMatrix::new(cols, rows, T::default());
        for (y, row) in data.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                matrix.set(x, y, val);
            }
        }
        matrix
    }

    #[inline]
    fn get_idx(&self, x: usize, y: usize) -> usize {
        self.cols * y + x
    }

    pub fn set(&mut self, x: usize, y: usize, value: &T) {
        let idx = self.get_idx(x, y);
        self.data[idx] = value.clone();
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let idx = self.get_idx(x, y);
        if idx >= self.len() {
            None
        } else {
            Some(&self.data[idx])
        }
    }

    pub fn get_row(&self, row_num: usize) -> Option<&[T]> {
        let start = self.get_idx(0, row_num);
        let end = self.get_idx(self.cols - 1, row_num);
        if start >= self.len() || end >= self.len() {
            None
        } else {
            Some(&self.data[start..=end])
        }
    }

    pub fn get_col(&self, col_num: usize) -> Option<Vec<T>> {
        let mut col = Vec::with_capacity(self.rows);
        for y in 0..self.rows {
            col.push(self.data[self.get_idx(col_num, y)].clone());
        }
        Some(col)
    }

    pub fn insert_row(&mut self, row_num: usize, row: &[T]) {
        let start = self.get_idx(0, row_num);
        let end = self.get_idx(self.cols - 1, row_num);
        if start >= self.len() || end >= self.len() {
            return;
        }
        row.iter()
            .enumerate()
            .for_each(|(idx, val)| self.data.insert(start + idx, val.clone()));
        self.rows += 1;
    }

    pub fn insert_col(&mut self, col_num: usize, col: &[T]) {
        self.cols += 1;
        for (y, val) in col.iter().enumerate() {
            self.data.insert(self.get_idx(col_num, y), val.clone());
        }
    }

    pub fn len(&self) -> usize {
        self.cols * self.rows
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_neighbors(&self, offsets: &[Coord], target: &Coord) -> Vec<Neighbor<T>> {
        offsets
            .iter()
            .map(|offset| offset.add(target))
            .filter_map(|coord| {
                if coord.x < 0 || coord.y < 0 {
                    None
                } else {
                    self.get(coord.x as usize, coord.y as usize)
                        .map(|val| (val.clone(), coord) as Neighbor<T>)
                }
            })
            .collect_vec()
    }

    pub fn get_direct_neighbors(&self, target: &Coord) -> Vec<Neighbor<T>> {
        #[rustfmt::skip]
            let offsets = [
                                   Coord::new(0, 1),
            Coord::new(1, 0),                        Coord::new(-1, 0),
                                   Coord::new(0, -1),
        ];
        self.get_neighbors(&offsets, target)
    }

    pub fn get_all_neighbors(&self, target: &Coord) -> Vec<Neighbor<T>> {
        #[rustfmt::skip]
        let offsets = [
            Coord::new(1, 1),  Coord::new(0, 1),  Coord::new(-1, 1),
            Coord::new(1, 0),                          Coord::new(-1, 0),
            Coord::new(1, -1), Coord::new(0, -1), Coord::new(-1, -1),
        ];
        self.get_neighbors(&offsets, target)
    }

    pub fn debug(&self, sep: &str) -> String {
        self.data
            .chunks(self.cols)
            .map(|row| {
                row.iter()
                    .map(|d| format!("{}", d))
                    .collect::<Vec<String>>()
                    .join(sep)
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn file_debug(&self, file_name: &str, sep: &str) -> std::io::Result<()> {
        let output = self.debug(sep);

        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("/workspaces/advent-of-code-rust/{}", file_name))?;
        f.write_all(output.as_bytes())?;
        f.flush()
    }

    pub fn iter(&self) -> AsciiMatrixIterator<T> {
        AsciiMatrixIterator {
            matrix: self,
            idx: 0,
        }
    }
}

impl<T: Debug> Display for AsciiMatrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self
            .data
            .chunks(self.cols)
            .map(|row| format!("{:?}", row))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", string)
    }
}

pub struct AsciiMatrixIterator<'a, T> {
    matrix: &'a AsciiMatrix<T>,
    idx: usize,
}
impl<'a, T: Clone + Display + Default> Iterator for AsciiMatrixIterator<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<(Coord, &'a T)> {
        if self.idx >= self.matrix.len() {
            None
        } else {
            let x = self.idx % self.matrix.cols;
            let y = self.idx / self.matrix.cols;
            let val = &self.matrix.data[self.idx];
            self.idx += 1;
            Some((Coord::new(x as isize, y as isize), val))
        }
    }
}
