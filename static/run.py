import qs
qs.hello()

def init():
    return [1,]

def update(state):
    print(state)
    state[0] += 1

def draw(state):
    qs.hello()

return {
    "init": init,
    "update": update,
    "draw": draw,
}
