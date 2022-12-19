with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")

commands = []

# Build the list of commands
for line in list(map(lambda ln: ln.removeprefix("turn "), data)):
    cmd = {}
    mode, start_coords, _, end_coords = line.split(" ")
    cmd["mode"] = mode
    cmd["start"] = tuple(map(int, start_coords.split(",")))
    cmd["end"] = tuple(map(int, end_coords.split(",")))
    commands.append(cmd)

# --- Part 1 ---
grid = [[False for __ in range(1000)] for _ in range(1000)]

for cmd in commands:
    for y in range(cmd["start"][1], cmd["end"][1] + 1):
        for x in range(cmd["start"][0], cmd["end"][0] + 1):
            if cmd["mode"] == "on":
                grid[y][x] = True
            elif cmd["mode"] == "off":
                grid[y][x] = False
            else:
                grid[y][x] = not grid[y][x]
print("Day 06, Part 01:", sum(list(map(sum, grid))))

del grid

# --- Part 2 ---
grid = [[0 for __ in range(1000)] for _ in range(1000)]

for cmd in commands:
    for y in range(cmd["start"][1], cmd["end"][1] + 1):
        for x in range(cmd["start"][0], cmd["end"][0] + 1):
            if cmd["mode"] == "on":
                grid[y][x] += 1
            elif cmd["mode"] == "off":
                grid[y][x] -= 1 if grid[y][x] >= 1 else 0
            else:
                grid[y][x] += 2

print("Day 06, Part 02:", sum(list(map(sum, grid))))
