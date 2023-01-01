with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")

coords = list(map(lambda d: tuple(map(int, d.split(","))), data))

cubes = set(coords)

DELTAS = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
]

# --- Part 1 ---

sa = 0
for c in coords:
    x, y, z = c
    for dx, dy, dz in DELTAS:  # Check every direction
        if (x + dx, y + dy, z + dz) not in cubes:
            sa += 1
print("Day 18, Part 01:", sa)

# --- Part 2 ---
from collections import deque

# Get the ranges for the values
ax = [r[0] for r in cubes]
ay = [r[1] for r in cubes]
az = [r[2] for r in cubes]
ranges = {"x": (min(ax), max(ax)), "y": (min(ay), max(ay)), "z": (min(az), max(az))}


def check_exterior(x: int, y: int, z: int) -> bool:
    q = deque()
    q.append((x, y, z))
    s = set()  # Used to track cubes that have been seen
    while q:
        cx, cy, cz = q.popleft()
        if (cx, cy, cz) in s:  # If this cube has been seen
            continue
        s.add((cx, cy, cz))
        # We should be checking exteriors only (the surrounding layer)
        if ((cx, cy, cz)) in cubes:
            continue
        if (
            cx > ranges["x"][1]
            or x < ranges["x"][0]
            or y > ranges["y"][1]
            or y < ranges["y"][0]
            or z > ranges["z"][1]
            or z < ranges["z"][0]
        ):
            return True
        for dx, dy, dz in DELTAS:
            q.append((cx + dx, cy + dy, cz + dz))
    return False


ex = 0
for c in coords:
    x, y, z = c
    for dx, dy, dz in DELTAS:
        if check_exterior(x + dx, y + dy, z + dz):
            ex += 1
print("Day 18, Part 02:", ex)
