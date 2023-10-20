use std::{cell::UnsafeCell, ops::Range};

pub trait Matrixer {
    fn size(&self) -> (usize, usize);
    fn at(&self, row: usize, col: usize) -> f64;
    fn sub_matrix(&mut self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrix;
}

pub trait MatrixerMut: Matrixer {
    fn at_mut(&mut self, row: usize, col: usize) -> &mut f64;
    fn sub_matrix_mut(&mut self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrixMut;
    fn into_four(&mut self) -> [SubMatrixMut; 4] {
        let n = self.size().0;
        let m = self.size().1;
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
}

pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrixer for Matrix {
    fn at(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }
    fn sub_matrix(&mut self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrix {
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

struct SubMatrix<'a> {
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
    fn sub_matrix(&mut self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrix {
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

struct SubMatrixMut<'a> {
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
    fn sub_matrix(&mut self, row_range: Range<usize>, col_range: Range<usize>) -> SubMatrix {
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

impl<'a> SubMatrixMut<'a> {
    fn at_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        &mut self.matrix.data[row * self.matrix.cols + col]
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
