def clamp(x, a, b):
    return max(min(b, x), a)

keypad = [[1, 2, 3],
          [4, 5, 6],
          [7, 8, 9]]
coords = [1, 1]
directions = {'U' : (0, -1), 'D': (0, 1), 'L': (-1, 0), 'R': (1, 0)}

for line in open("input-02.txt"):
    for char in line.rstrip("\n"):
        coords[0] += directions[char][0]
        coords[1] += directions[char][1]
        coords = [clamp(coords[0], 0, 2),
                  clamp(coords[1], 0, 2)]

    print keypad[coords[1]][coords[0]]
