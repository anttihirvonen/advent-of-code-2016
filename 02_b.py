def clamp(x, a, b):
    return max(min(b, x), a)

keypad = [[0,  0,   1,   0,  0],
          [0,  2,   3,   4,  0],
          [5,  6,   7,   8,  9],
          [0, 'A', 'B', 'C', 0],
          [0,  0,  'D',  0,  0]]
coords = [0, 2]
directions = {'U' : (0, -1), 'D': (0, 1), 'L': (-1, 0), 'R': (1, 0)}

for line in open("input-02.txt"):
    for char in line.rstrip("\n"):
        new_coords = coords[:]
        new_coords[0] += directions[char][0]
        new_coords[1] += directions[char][1]
        new_coords = [clamp(new_coords[0], 0, 4),
                      clamp(new_coords[1], 0, 4)]
        if keypad[new_coords[1]][new_coords[0]] != 0:
            coords = new_coords

    print keypad[coords[1]][coords[0]]
