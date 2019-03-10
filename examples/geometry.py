""" Draw some multi-colored geometry to the screen. """
import qs
from common import *

def init():
    pass

def update(_state):
    pass

def draw(_state):
    # Remove any lingering artifacts from the previous frame
    qs.clear(WHITE)
    # Draw a rectangle with a top-left corner at (100, 100) and a width and height of 32 with
    # a blue background
    qs.rect([[100,100], [32,32]], color=BLUE)
    # Draw another rectangle, rotated by 45 degrees, with a z-height of 10
    qs.rect([[400, 300], [32, 32]], color=BLUE, transform=rotate(45), z=10)
    # Draw a circle with its center at (400, 300) and a radius of 100, with a background of
    # green
    qs.circ( [400, 300], 100., color=GREEN)
    # Draw a line with a thickness of 2 pixels, a red background,
    # and a z-height of 5
    qs.line([[50, 80], [600, 450]], thickness=2., color=RED, z=5)
    # Draw a triangle with a red background, rotated by 45 degrees, and scaled down to half
    # its size
    qs.triangle([[500, 50], [450, 100], [650, 150]], color=RED, transform=matmul(rotate(45), scale(0.5, 0.5)), z=0)

def event(state, event):
    pass
