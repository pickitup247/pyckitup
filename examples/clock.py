import qs
from common import *

def init():
    qs.init_sounds([ ["click", "click.wav"] ])
    return {
        "elapsed": 0,
        "hours": 0,
        "minutes": 0,
        "seconds": 0,
    }

def onload(_):
    qs.set_update_rate(1000)

def update(state):
    state["elapsed"] += qs.update_rate()
    elapsed = state["elapsed"]
    state["seconds"] = (elapsed / 1000.) % 60.
    state["minutes"] = ((elapsed / 1000.) / 60.) % 60.
    state["hours"] = ((elapsed / 1000.) / 60. / 24.) % 24.

    qs.sound("click")

def draw(state):
    qs.clear(WHITE)

    # draw the frame
    qs.circ([400, 300], 203, color=BLACK)
    qs.circ([400, 300], 200, color=WHITE)
    # draw the hour markers
    for i in range(1, 13):
        angle = 360. * ((i + 9.) * 2. / 24.)
        rad = angle * 3.14 / 180
        pos = [ math.sin(rad) * 200. + 400., math.cos(rad) * 200. + 300. ]
        qs.line([[400, 300], pos], thickness=5, color=BLACK)
    qs.circ([400, 300], 180, color=WHITE)

    hour_angle = 360. * ((state["hours"] + 9.) * 2. / 24.) * 3.14 / 180
    minute_angle = 360. * ((state["minutes"] + 45.) / 60.) * 3.14 / 180
    second_angle = 360. * ((state["seconds"] + 45.) / 60.) * 3.14 / 180

    hour_pos =   [math.cos(hour_angle  ) * 150. + 400, math.sin(hour_angle)* 150 + 300]
    min_pos =    [math.cos(minute_angle) * 180. + 400, math.sin(minute_angle) * 180+ 300]
    second_pos = [math.cos(second_angle) * 180. + 400, math.sin(second_angle) * 180. + 300]

    qs.line([[400, 300], hour_pos], thickness=10, color=BLACK)
    qs.line([[400, 300], min_pos], thickness=5, color=BLUE)
    qs.line([[400, 300], second_pos], thickness=3, color=RED)
