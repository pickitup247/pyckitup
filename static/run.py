import qs
qs.hello()

test = 0

def init():
    return {"id": 1}

def update(state):
    print(state)
    state['id'] += 1
    test += 1
    print(test)

def draw(state):
    qs.hello()

return {
    "init": init,
    "update": update,
    "draw": draw,
}
