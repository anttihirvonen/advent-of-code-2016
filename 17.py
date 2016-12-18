import md5

INPUT = "vkjiggvb"

def make_move(path, dir):
    path[0] += {'U': -1j, 'R': 1, 'D': 1j, 'L': -1}[dir]

    in_limits = lambda c: int(c) >= 0 and int(c) < 4

    if in_limits(path[0].real) and in_limits(path[0].imag):
        code = md5.new(INPUT + path[1]).hexdigest()
        status = code[{'U': 0, 'D': 1, 'L': 2, 'R': 3}[dir]]

        if status in ['b', 'c', 'd', 'e', 'f']:
            return [[path[0], path[1] + dir]]

    return []

def generate_paths():
    paths = [[0 + 0, ""]]

    while len(paths) > 0:
        path = paths.pop(0)

        if path[0] == 3 + 3j:
            yield path
            continue

        for d in ['U', 'D', 'R', 'L']:
            paths += make_move(path[:], d)

print "A:", generate_paths().next()[1]
print "B:", sorted([len(path[1]) for path in generate_paths()])[-1]
