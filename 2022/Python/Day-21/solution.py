with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")

# --- Part 1 ---
monkeys = {}

# Parse the data
for d in data:
    m, j = d.split(": ")
    if len(j.split(" ")) != 3:
        lhs, op, rhs = int(j), "+", 0
    else:
        lhs, op, rhs = j.split(" ")
    monkeys[m] = (lhs, op, rhs)

# Stores the values for each monkey
monkey_values = {}


def getMonkey(m: str) -> int:
    monkey = monkeys[m]
    if isinstance(monkey[0], int):
        monkey_values[m] = monkey
        return monkey[0]
    else:
        val_l, val_r = getMonkey(monkey[0]), getMonkey(monkey[2])
        if monkey[1] == "+":
            monkey_values[m] = val_l + val_r
            return val_l + val_r
        if monkey[1] == "-":
            monkey_values[m] = val_l - val_r
            return val_l - val_r
        if monkey[1] == "*":
            monkey_values[m] = val_l * val_r
            return val_l * val_r
        else:
            monkey_values[m] = val_l / val_r
            return val_l / val_r


for m in monkeys:
    if m not in monkey_values:
        getMonkey(m)
print("Day 21, Part 01:", int(monkey_values["root"]))

# --- Part 2 ---

# Idek...
