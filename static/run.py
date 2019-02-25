import qs

def init():
    return [1,]

def update(state):
    print(state)
    state[0] += 1

def draw(state):
    qs.draw_rect()
