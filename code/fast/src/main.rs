mod matrix;
use matrix::*;

fn matrix_mult(a: &Matrix, b: &Matrix) -> Matrix {
    let (n, _) = a.size();
    let mut c = Matrix::zero(n, n);
    matrix_mult_rec(a, b, &mut c, n);
    c
}

fn matrix_mult_rec(a: &impl Matrixer, b: &impl Matrixer, c: &mut impl MatrixerMut, n: usize) {
    if n == 1 {
        *c.at_mut(0, 0) += a.at(0, 0) * b.at(0, 0);
        return;
    }

    let n = n / 2;

    let [a11, a12, a21, a22] = a.into_four();
    let [b11, b12, b21, b22] = b.into_four();
    let [mut c11, mut c12, mut c21, mut c22] = c.into_four_mut();

    matrix_mult_rec(&a11, &b11, &mut c11, n);
    matrix_mult_rec(&a11, &b12, &mut c12, n);
    matrix_mult_rec(&a21, &b11, &mut c21, n);
    matrix_mult_rec(&a21, &b12, &mut c22, n);
    matrix_mult_rec(&a12, &b21, &mut c11, n);
    matrix_mult_rec(&a12, &b22, &mut c12, n);
    matrix_mult_rec(&a22, &b21, &mut c21, n);
    matrix_mult_rec(&a22, &b22, &mut c22, n);
}

fn strassen_matrix_mult(a: &Matrix, b: &Matrix) -> Matrix {
    let (n, _) = a.size();
    let mut c = Matrix::zero(n, n);
    let mut buffer = Matrix::zero(n * 2, n * 4);
    strassen_matrix_mult_rec(a, b, &mut c, &mut buffer, n);
    c
}

fn strassen_matrix_mult_rec(
    a: &impl Matrixer,
    b: &impl Matrixer,
    c: &mut impl MatrixerMut,
    buffer: &mut impl MatrixerMut,
    n: usize,
) {
    if n == 1 {
        *c.at_mut(0, 0) += a.at(0, 0) * b.at(0, 0);
        return;
    }

    let n = n / 2;

    buffer.set_zero();

    let [mut buffer_left, mut buffer_right] = buffer.cols_into_half_mut();

    let [a11, a12, a21, a22] = a.into_four();
    let [b11, b12, b21, b22] = b.into_four();
    let [mut c11, mut c12, mut c21, mut c22] = c.into_four_mut();

    let [mut buf11, mut buf12, mut buf21, mut buf22] = buffer_left.into_four_mut();
    let [mut s1, mut s2, mut s3, mut s4] = buf11.into_four_mut();
    let [mut s5, mut s6, mut s7, mut s8] = buf12.into_four_mut();
    let [mut s9, mut s10, mut p1, mut p2] = buf21.into_four_mut();
    let [mut p3, mut p4, mut p5, mut p6] = buf22.into_four_mut();

    let [mut useless_buffer, mut rec_buffer] = buffer_right.rows_into_half_mut();
    let [mut p7, _, _, _] = useless_buffer.into_four_mut();

    s1.add(&b12);
    s1.subtract(&b22);

    s2.add(&a11);
    s2.add(&a12);

    s3.add(&a21);
    s3.add(&a22);

    s4.add(&b21);
    s4.subtract(&b11);

    s5.add(&a11);
    s5.add(&a22);

    s6.add(&b11);
    s6.add(&b22);

    s7.add(&a12);
    s7.subtract(&a22);

    s8.add(&b21);
    s8.add(&b22);

    s9.add(&a11);
    s9.subtract(&a21);

    s10.add(&b11);
    s10.add(&b12);

    strassen_matrix_mult_rec(&a11, &s1, &mut p1, &mut rec_buffer, n);
    strassen_matrix_mult_rec(&s2, &b22, &mut p2, &mut rec_buffer, n);
    strassen_matrix_mult_rec(&s3, &b11, &mut p3, &mut rec_buffer, n);
    strassen_matrix_mult_rec(&a22, &s4, &mut p4, &mut rec_buffer, n);
    strassen_matrix_mult_rec(&s5, &s6, &mut p5, &mut rec_buffer, n);
    strassen_matrix_mult_rec(&s7, &s8, &mut p6, &mut rec_buffer, n);
    strassen_matrix_mult_rec(&s9, &s10, &mut p7, &mut rec_buffer, n);

    c11.add(&p5);
    c11.add(&p4);
    c11.subtract(&p2);
    c11.add(&p6);

    c12.add(&p1);
    c12.add(&p2);

    c21.add(&p3);
    c21.add(&p4);

    c22.add(&p5);
    c22.add(&p1);
    c22.subtract(&p3);
    c22.subtract(&p7);
}

fn main() {
    let n = 4096;
    let a = Matrix::random(n, n);
    let b = Matrix::random(n, n);

    let now = std::time::Instant::now();
    let c = matrix_mult(&a, &b);
    println!("{} ms", now.elapsed().as_millis());
    println!("{}", c.at(0, 0));

    let now = std::time::Instant::now();
    let c = strassen_matrix_mult(&a, &b);
    println!("{} ms", now.elapsed().as_millis());
    println!("{}", c.at(0, 0));
}
