data = None

with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")

data = """498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9""".strip().split(
    "\n"
)

# --- Part 1 ---

rocks = set()

for line in data:
    prev = None
    for point in line.split(" -> "):
        x, y = tuple(map(lambda p: int(p), point.split(",")))
        if prev is not None:
            dx = x - prev[0]
            dy = y - prev[1]
            length = max(abs(dx), abs(dy))
            for i in range(length):
                x_ = prev[0] + i * (1 if dx > 0 else (-1 if dx < 0 else 0))
                y_ = prev[1] + i * (1 if dy > 0 else (-1 if dy < 0 else 0))
                rocks.add((x_, y_))
        prev = (x, y)


def sim_p1() -> int:
    for n in range(10000):
        spot = (500, 0)
        while True:
            if spot[1] > 300:
                return n


print()
