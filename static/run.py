import qs

def init():
    return [[1, 1], [1, 1]]

def update(state):
    state[0][0] += 1
    state[0][1] += 1
    state[1][0] += 1
    state[1][1] += 1

def draw(state):
    color = [1.,1.,1.,1.]
    transform = { "rotate": 45 }
    z = 10
    qs.rect(state, color, transform=transform, z=z)
