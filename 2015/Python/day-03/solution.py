with open("input.txt", "r") as f:
    data = f.read().strip()

# --- Part 1 ---
x, y = 0, 0
houses = set()
houses.add((x, y))

for d in list(data):
    if d == "^":
        y += 1
    elif d == ">":
        x += 1
    elif d == "v":
        y -= 1
    elif d == "<":
        x -= 1
    houses.add((x, y))
print("D03P01:", len(houses))

# --- Part 2 ---
sx, sy = 0, 0
rx, ry = 0, 0
houses.clear()
houses.add((0, 0))

for i, d in enumerate(list(data)):
    mod_x, mod_y = 0, 0
    if d == "^":
        mod_y += 1
    elif d == ">":
        mod_x += 1
    elif d == "v":
        mod_y -= 1
    elif d == "<":
        mod_x -= 1
    if i % 2 == 0:
        sx += mod_x
        sy += mod_y
        houses.add((sx, sy))
    else:
        rx += mod_x
        ry += mod_y
        houses.add((rx, ry))
print("D03P02:", len(houses))
