from typing import List
import numpy as np
import time


def matrix_mult_rec(a, b, c, n: int):
    if n == 1:
        c[0][0] += a[0][0] * b[0][0]
        return

    n = n // 2

    a11 = a[:n, :n]
    a12 = a[:n, n:]
    a21 = a[n:, :n]
    a22 = a[n:, n:]

    b11 = b[:n, :n]
    b12 = b[:n, n:]
    b21 = b[n:, :n]
    b22 = b[n:, n:]

    c11 = c[:n, :n]
    c12 = c[:n, n:]
    c21 = c[n:, :n]
    c22 = c[n:, n:]

    matrix_mult_rec(a11, b11, c11, n)
    matrix_mult_rec(a11, b12, c12, n)
    matrix_mult_rec(a21, b11, c21, n)
    matrix_mult_rec(a21, b12, c22, n)
    matrix_mult_rec(a12, b21, c11, n)
    matrix_mult_rec(a12, b22, c12, n)
    matrix_mult_rec(a22, b21, c21, n)
    matrix_mult_rec(a22, b22, c22, n)


def matrix_mult(a, b, n):
    c = np.zeros((n, n))
    matrix_mult_rec(a, b, c, n)
    return c


def strassen_matrix_mult_rec(a, b, c, n: int):
    if n == 1:
        c[0][0] += a[0][0] * b[0][0]
        return

    n = n // 2

    a11 = a[:n, :n]
    a12 = a[:n, n:]
    a21 = a[n:, :n]
    a22 = a[n:, n:]

    b11 = b[:n, :n]
    b12 = b[:n, n:]
    b21 = b[n:, :n]
    b22 = b[n:, n:]

    c11 = c[:n, :n]
    c12 = c[:n, n:]
    c21 = c[n:, :n]
    c22 = c[n:, n:]

    s1 = b12 - b22
    s2 = a11 + a12
    s3 = a21 + a22
    s4 = b21 - b11
    s5 = a11 + a22
    s6 = b11 + b22
    s7 = a12 - a22
    s8 = b21 + b22
    s9 = a11 - a21
    s10 = b11 + b12

    p1 = strassen_matrix_mult(a11, s1, n)
    p2 = strassen_matrix_mult(s2, b22, n)
    p3 = strassen_matrix_mult(s3, b11, n)
    p4 = strassen_matrix_mult(a22, s4, n)
    p5 = strassen_matrix_mult(s5, s6, n)
    p6 = strassen_matrix_mult(s7, s8, n)
    p7 = strassen_matrix_mult(s9, s10, n)

    c11 += p5 + p4 - p2 + p6
    c12 += p1 + p2
    c21 += p3 + p4
    c22 += p5 + p1 - p3 - p7


def strassen_matrix_mult(a, b, n):
    c = np.zeros((n, n))
    strassen_matrix_mult_rec(a, b, c, n)
    return c


def benchmark(func, *args):
    start = time.time()
    func(*args)
    end = time.time()
    print(f"{func.__name__} took {end - start} seconds")


n = 128
a = np.random.randint(10, size=(n, n))
b = np.random.randint(10, size=(n, n))

benchmark(matrix_mult, a, b, n)
benchmark(strassen_matrix_mult, a, b, n)
