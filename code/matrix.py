def matrix_mult_rec(a: [[int]], b: [[int]], c: [[int]], n: int):
    if n == 1:
        c[0][0] = a[0][0] * b[0][0]
        return

    n = n // 2
    print(n)

    a11 = a[:n][:n]
    a12 = a[:n][n:]
    a21 = a[n:][:n]
    a22 = a[n:][n:]
    b11 = b[:n][:n]
    b12 = b[:n][n:]
    b21 = b[n:][:n]
    b22 = b[n:][n:]
    c11 = c[:n][:n]
    c12 = c[:n][n:]
    c21 = c[n:][:n]
    c22 = c[n:][n:]

    print(a11, a12, a21, a22)

    matrix_mult_rec(a11, b11, c11, n)
    matrix_mult_rec(a12, b21, c11, n)
    matrix_mult_rec(a11, b12, c12, n)
    matrix_mult_rec(a12, b22, c12, n)
    matrix_mult_rec(a21, b11, c21, n)
    matrix_mult_rec(a22, b21, c21, n)
    matrix_mult_rec(a21, b12, c22, n)
    matrix_mult_rec(a22, b22, c22, n)

    return c


print(
    matrix_mult_rec(
        [
            [1, 2],
            [3, 1],
        ],
        [
            [0, 1],
            [1, 0],
        ],
        [
            [0, 0],
            [0, 0],
        ],
        2,
    )
)
# should print
# [
#     [19, 22],
#     [43, 50],
# ]
