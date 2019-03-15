import qs
from common import *

def init():
    qs.init_fonts([
        ("precious", "Precious.ttf")
    ])
    return None

def draw(_):
    qs.clear(WHITE)
    qs.text("hello world", font="precious", p0=[0,0], pt=100, color=RED)
