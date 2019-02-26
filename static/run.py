import qs

red = [1., 0., 0., 1.]
green = [0., 1., 0., 1.]
blue = [0., 0., 1., 1.]

def init():
    qs.init_images([
        ["crab", "crab.png"],
    ])
    qs.init_anims([
        ["crab-left", "crab-left.png", 36, 27, 1.],
        ["crab-up", "crab-up.png", 36, 27, 1.]
    ])
    qs.init_sounds([
        # ["click", "click.wav"]
    ])
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

    qs.rect(
        [state["p0"],state["p1"]],
        blue,
        z=1,
    )

    # qs.circ(
    #     [state["p0"], 30.],
    #     red
    # )
    # qs.triangle(
    #     [
    #         state["p0"],
    #         state["p1"],
    #         state["p2"],
    #     ],
    #     green
    # )

    # qs.sprite("crab", [state["p0"], state["p1"]])
    # qs.anim("crab-left", [state["p0"], state["p1"]])
    qs.anim("crab-up", [state["p0"], state["p1"]], z=1)
