# This is purely for learning at this point.
# Thanks https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/15.py
# for this slow, inefficient solution that I may try to optimize at some point.
with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")

# --- Part 1 ---
def get_manhattan(sx: int, sy: int, bx: int, by: int) -> int:
    """Calculates the Manhattan Distance between two given points."""
    return abs(sx - bx) + abs(sy - by)


sensors = {}
beacons = {}

for line in data:
    sp, bp = line.split(": ")  # [Sensor part, Beacon part]
    sp = sp.removeprefix("Sensor at ").split(", ")
    bp = bp.removeprefix("closest beacon is at ").split(", ")
    sx, sy = int(sp[0].removeprefix("x=")), int(sp[1].removeprefix("y="))
    bx, by = int(bp[0].removeprefix("x=")), int(bp[1].removeprefix("y="))
    sensors[(sx, sy)] = {
        "beacon": (bx, by),
        "dist": get_manhattan(sx, sy, bx, by),
    }
    beacon = beacons.get((bx, by), [])
    beacon.append((sx, sy))
    beacons[(bx, by)] = beacon

# Get the min and max x values that we will have to check
min_x = min(
    min([pos[0] - sensors[pos]["dist"] for pos in sensors.keys()]),
    min([pos[0] for pos in beacons.keys()]),
)
max_x = max(
    max([pos[0] + sensors[pos]["dist"] for pos in sensors.keys()]),
    max([pos[0] for pos in beacons.keys()]),
)


def check_if_valid(x: int, y: int) -> bool:
    """Checks if a given point is valid (is the point in the search radius
    for a sensor?)."""
    for sensor in sensors.items():
        if abs(x - sensor[0][0]) + abs(y - sensor[0][1]) <= sensor[1]["dist"]:
            return False
    return True


def check_row(row: int) -> int:
    """Checks a row to find how many cells are not covered by sensors."""
    count = 0
    for x in range(min_x, max_x + 1):
        if (x, row) not in beacons.keys() and not check_if_valid(x, row):
            count += 1
    return count


print("D15P1:", check_row(2000000))

# --- Part 2 ---
def check_for_hidden_beacon(max_radius) -> int:
    """Checks the grid from 0 up to the `max_radius` to find a position that
    may not be covered that is the actual hidden beacon."""
    for pos, this in sensors.items():
        for dx in range(this["dist"] + 2):
            dy = this["dist"] + 1 - dx
            for mod_x, mod_y in [(-1, -1), (-1, 1), (1, 1), (1, -1)]:
                x = pos[0] + dx * mod_x
                y = pos[1] + dy * mod_y
                if not (0 <= x <= max_radius and 0 <= y <= max_radius):
                    continue
                if check_if_valid(x, y):
                    return x * 4000000 + y
    return 0


print(check_for_hidden_beacon(4000000))
