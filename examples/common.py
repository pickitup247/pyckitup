import math

RED = [1., 0., 0., 1.]
GREEN = [0., 1., 0., 1.]
BLUE = [0., 0., 1., 1.]
WHITE = [1., 1., 1., 1.]
BLACK = [0., 0., 0., 1.]

def rotate(deg):
    theta = deg * 3.14 / 180.
    c = math.cos(theta)
    s = math.sin(theta)
    return [
        [c,-s, 0],
        [s, c, 0],
        [0, 0, 1]
    ]

def translate(x, y):
    return [
        [1, 0, x],
        [0, 1, y],
        [0, 0, 1]
    ]

def scale(x, y):
    return [
        [x, 0, 0],
        [0, y, 0],
        [0, 0, 1]
    ]

def matmul(X, Y):
    return [[sum(a*b for a,b in zip(X_row,Y_col)) for Y_col in zip(*Y)] for X_row in X]
