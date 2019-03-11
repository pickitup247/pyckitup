""" Draw some multi-colored geometry to the screen. """
import qs
from common import *

def init():
    return [[0, 0], [800, 600]]

def update(state):
    keys = qs.keyboard_bool()
    if keys["Left"] or keys["A"]:
        state[0][0] += 4
    if keys["Right"] or keys["D"]:
        state[0][0] -= 4
    if keys["Up"] or keys["W"]:
        state[0][1] += 4
    if keys["Down"] or keys["S"]:
        state[0][1] -= 4
    qs.set_view(state)

def draw(_state):
    # Remove any artifacts from the previous frame
    qs.clear(WHITE)
    # Draw a blue rectangle with a top-left corner at (100, 100) and a width and height of 32
    qs.rect([[100,100], [32,32]], color=BLUE)
    # Draw a blue rectangle, rotated by 45 degrees, with a z-height of 10
    qs.rect([[400, 300], [32, 32]], color=BLUE, transform=rotate(45), z=10)
    # Draw a green circle with its center at (400, 300) and a radius of 100
    qs.circ( [400, 300], 100., color=GREEN)
    # Draw a red line with thickness of 2 pixels and z-height of 5
    qs.line([[50, 80], [600, 450]], thickness=2., color=RED, z=5)
    # Draw a red triangle rotated by 45 degrees, and scaled down to half
    qs.triangle([[500, 50], [450, 100], [650, 150]], color=RED, transform=matmul(rotate(45), scale(0.5, 0.5)), z=0)

def event(state, event):
    pass
