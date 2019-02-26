import qs

red = [1., 0., 0., 1.]
green = [0., 1., 0., 1.]
blue = [0., 0., 1., 1.]

def init():
    return {
        "p0": [1., 1.],
        "p1": [100., 100.],
        "p2": [300., 50.],
    }

def update(state):
    state["p0"][0] += 1.
    state["p0"][1] += 1.

def draw(state):
    transform = [("rotate", 45)]
    z = 10

    qs.rect(
        [state["p0"],state["p1"]],
        blue
    )

    qs.circ(
        [state["p0"], 30.],
        red
    )

    qs.triangle(
        [
            state["p0"],
            state["p1"],
            state["p2"],
        ],
        green
    )
