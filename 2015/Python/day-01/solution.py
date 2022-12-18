with open("input.txt", "r") as f:
    data = f.read().strip()

# --- Part 1 ---
floors_up, floors_down = data.count("("), data.count(")")
print("D01P01:", floors_up + (-1 * floors_down))

# --- Part 2 ---
floor = 0
for i, c in enumerate(list(data)):
    if c == "(":
        floor += 1
    elif c == ")":
        floor -= 1
    if floor == -1:
        print("D01P02:", i + 1)
        break
