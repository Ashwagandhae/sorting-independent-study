use std::{cell::UnsafeCell, ops::Range};

pub trait Matrixer: std::fmt::Debug + std::fmt::Display {
    fn size(&self) -> (usize, usize);
    fn at(&self, row: usize, col: usize) -> f64;
    fn sub_matrix(&self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrix;
    fn into_four(&self) -> [SubMatrix; 4] {
        let (n, m) = self.size();
        [
            self.sub_matrix(0..n / 2, 0..m / 2),
            self.sub_matrix(0..n / 2, m / 2..m),
            self.sub_matrix(n / 2..n, 0..m / 2),
            self.sub_matrix(n / 2..n, m / 2..m),
        ]
    }
    fn rows_into_half(&mut self) -> [SubMatrix; 2] {
        let (n, m) = self.size();
        [
            self.sub_matrix(0..n / 2, 0..m),
            self.sub_matrix(n / 2..n, 0..m),
        ]
    }
    fn cols_into_half(&mut self) -> [SubMatrix; 2] {
        let (n, m) = self.size();
        [
            self.sub_matrix(0..n, 0..m / 2),
            self.sub_matrix(0..n, m / 2..m),
        ]
    }
}

pub trait MatrixerMut: Matrixer {
    fn at_mut(&mut self, row: usize, col: usize) -> &mut f64;
    fn sub_matrix_mut(&mut self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrixMut;
    fn into_four_mut(&mut self) -> [SubMatrixMut; 4] {
        let (n, m) = self.size();
        unsafe {
            let cell = UnsafeCell::new(self);
            [
                (&mut *cell.get()).sub_matrix_mut(0..n / 2, 0..m / 2),
                (&mut *cell.get()).sub_matrix_mut(0..n / 2, m / 2..m),
                (&mut *cell.get()).sub_matrix_mut(n / 2..n, 0..m / 2),
                (&mut *cell.get()).sub_matrix_mut(n / 2..n, m / 2..m),
            ]
        }
    }
    fn rows_into_half_mut(&mut self) -> [SubMatrixMut; 2] {
        let (n, m) = self.size();
        unsafe {
            let cell = UnsafeCell::new(self);
            [
                (&mut *cell.get()).sub_matrix_mut(0..n / 2, 0..m),
                (&mut *cell.get()).sub_matrix_mut(n / 2..n, 0..m),
            ]
        }
    }
    fn cols_into_half_mut(&mut self) -> [SubMatrixMut; 2] {
        let (n, m) = self.size();
        unsafe {
            let cell = UnsafeCell::new(self);
            [
                (&mut *cell.get()).sub_matrix_mut(0..n, 0..m / 2),
                (&mut *cell.get()).sub_matrix_mut(0..n, m / 2..m),
            ]
        }
    }
    fn add(&mut self, other: &impl Matrixer) {
        for row in 0..self.size().0 {
            for col in 0..self.size().1 {
                *self.at_mut(row, col) += other.at(row, col);
            }
        }
    }
    fn subtract(&mut self, other: &impl Matrixer) {
        for row in 0..self.size().0 {
            for col in 0..self.size().1 {
                *self.at_mut(row, col) -= other.at(row, col);
            }
        }
    }
    fn set_zero(&mut self) {
        for row in 0..self.size().0 {
            for col in 0..self.size().1 {
                *self.at_mut(row, col) = 0.;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrixer for Matrix {
    fn at(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }
    fn sub_matrix(&self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrix {
        SubMatrix {
            matrix: self,
            row_start: row_range.start,
            row_end: row_range.end,
            col_start: col_range.start,
            col_end: col_range.end,
        }
    }
    fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

use rand::prelude::*;
impl Matrix {
    pub fn zero(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }
    pub fn from_vec(data: Vec<f64>, rows: usize, cols: usize) -> Self {
        Self { data, rows, cols }
    }
    pub fn random(rows: usize, cols: usize) -> Self {
        Self {
            data: (0..(rows * cols)).map(|_| rand::random::<f64>()).collect(),
            rows,
            cols,
        }
    }
}

impl MatrixerMut for Matrix {
    fn at_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.data[row * self.cols + col]
    }
    fn sub_matrix_mut(&mut self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrixMut {
        SubMatrixMut {
            matrix: self,
            row_start: row_range.start,
            row_end: row_range.end,
            col_start: col_range.start,
            col_end: col_range.end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubMatrix<'a> {
    matrix: &'a Matrix,
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
}

impl<'a> Matrixer for SubMatrix<'a> {
    fn at(&self, row: usize, col: usize) -> f64 {
        self.matrix.at(row + self.row_start, col + self.col_start)
    }
    fn sub_matrix(&self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrix {
        SubMatrix {
            matrix: self.matrix,
            row_start: self.row_start + row_range.start,
            row_end: self.row_start + row_range.end,
            col_start: self.col_start + col_range.start,
            col_end: self.col_start + col_range.end,
        }
    }
    fn size(&self) -> (usize, usize) {
        (self.row_end - self.row_start, self.col_end - self.col_start)
    }
}

#[derive(Debug)]
pub struct SubMatrixMut<'a> {
    matrix: &'a mut Matrix,
    row_start: usize,
    row_end: usize,
    col_start: usize,
    col_end: usize,
}

impl<'a> Matrixer for SubMatrixMut<'a> {
    fn at(&self, row: usize, col: usize) -> f64 {
        self.matrix.at(row + self.row_start, col + self.col_start)
    }
    fn sub_matrix(&self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrix {
        SubMatrix {
            matrix: self.matrix,
            row_start: self.row_start + row_range.start,
            row_end: self.row_start + row_range.end,
            col_start: self.col_start + col_range.start,
            col_end: self.col_start + col_range.end,
        }
    }
    fn size(&self) -> (usize, usize) {
        (self.row_end - self.row_start, self.col_end - self.col_start)
    }
}

impl<'a> MatrixerMut for SubMatrixMut<'a> {
    fn at_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        self.matrix
            .at_mut(row + self.row_start, col + self.col_start)
    }
    fn sub_matrix_mut(&mut self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrixMut {
        SubMatrixMut {
            matrix: self.matrix,
            row_start: self.row_start + row_range.start,
            row_end: self.row_start + row_range.end,
            col_start: self.col_start + col_range.start,
            col_end: self.col_start + col_range.end,
        }
    }
}

fn matrix_display(matrix: &impl Matrixer, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (n, m) = matrix.size();
    for i in 0..n {
        for j in 0..m {
            write!(f, "{} ", matrix.at(i, j))?;
        }
        writeln!(f)?;
    }

    Ok(())
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        matrix_display(self, f)
    }
}
impl<'a> std::fmt::Display for SubMatrix<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        matrix_display(self, f)
    }
}
impl<'a> std::fmt::Display for SubMatrixMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        matrix_display(self, f)
    }
}
