import qs
from common import *

def init():
    qs.init_sounds([ ["click", "click.wav"] ])
    return None

def draw(_):
    qs.circ([10, 10], 5, color=RED)

def event(_, evt):
    if evt["event"] == "mouse_button" \
        and evt["button"] == "Left" \
        and evt["state"] == "Pressed":
        qs.sound("click")
