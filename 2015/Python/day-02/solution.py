with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")

# --- Part 1 ---
dimens = [
    (int(d[0]), int(d[1]), int(d[2])) for d in list(map(lambda d: d.split("x"), data))
]
total = 0
for d in dimens:
    l, h, w = d
    sides = [l * w, w * h, h * l]
    total += sum(list(map(lambda s: 2 * s, sides))) + min(sides)
print("D02P01:", total)

# --- Part 2 ---
ribbon = 0
for d in dimens:
    l, h, w = d
    perims = [2 * l + 2 * h, 2 * l + 2 * w, 2 * w + 2 * h]
    ribbon += min(perims) + (l * w * h)
print("D02P02:", ribbon)
