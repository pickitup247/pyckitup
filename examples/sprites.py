import qs
from common import *

def init():
    qs.init_sprites([
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
    state["p0"][0] += 0.5
    state["p0"][1] += 0.5
    state["deg"] += 1.

def draw(state):
    p0 = state["p0"]
    p1 = state["p1"]
    transform = rotate(state["deg"])
    # qs.triangle( [ p0, p1, state["p2"]], color=BLUE, transform=transform, z=0)
    # qs.circ( p0, 30., color=RED)
    # qs.sprite("crab", rect=[p0, p1], z=1)
    # qs.sprite("crab", p0=p0, z=1)
    qs.anim("crab-left", rect=[p0, p1], z=4)

def event(state, event):
    if event["event"] == "mouse_moved":
        state["p0"][0] = event["x"]
        state["p0"][1] = event["y"]