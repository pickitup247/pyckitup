import qs
import math

red = [1., 0., 0., 1.]
green = [0., 1., 0., 1.]
blue = [0., 0., 1., 1.]

def rotate(deg):
    theta = deg * 3.14 / 180.
    c = math.cos(theta)
    s = math.sin(theta)
    return [
        [c,-s, 0],
        [s, c, 0],
        [0, 0, 1]
    ]


def init():
    qs.init_images([
        ["crab", "crab.png"],
    ])
    qs.init_anims([
        ["crab-left", "crab-left.png", 36, 27, 1.],
        ["crab-up", "crab-up.png", 36, 27, 1.]
    ])
    qs.init_sounds([
        ["click", "click.wav"]
    ])
    return {
        "p0": [1., 1.],
        "p1": [100., 100.],
        "p2": [300., 50.],
        "deg": 0.,
    }

def update(state):
    state["p0"][0] += 1.
    state["p0"][1] += 1.
    state["deg"] += 1.

def draw(state):
    p0 = state["p0"]
    p1 = state["p1"]

    # qs.circ( [p0, 30.], red)
    # qs.triangle( [ p0, p1, state["p2"], ], green)
    qs.sprite("crab", [p0, p1], z=1)
    qs.anim("crab-left", [p0, p1], z=1)

    transform = rotate(state["deg"])
    # qs.rect( [p0, p1], blue, transform=transform, z=0)
    # qs.anim("crab-up", [p0, p1], transform=transform, z=1)


def event(state, event):
    if event["event"] == "mouse_moved":
        state["p0"][0] = event["x"]
        state["p0"][1] = event["y"]
    elif event["event"] == "mouse_button":
        print(qs.mouse_pos())
    else:
        print(event)
