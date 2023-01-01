# Get the input
input = None

with open("input.txt", "r") as f:
    input = f.read().split("\n")

# --- Problem 1 ---
# Split each string in half
sacks = []  # [[sack half 1, sack half 2], ...]
for s in input:
    sacks.append([s[: len(s) // 2], s[len(s) // 2 :]])

sims = []  # Stores chars found to be similar

for sack in sacks:
    for c in list(sack[0]):
        if c in list(sack[1]):
            sims.append(c)
            break


sim_vals = []  # Stores the numerical value for each charcter

# Convert chars to their values
for sim in sims:
    if sim.islower():
        sim_vals.append(ord(sim) - 96)
    else:
        sim_vals.append(ord(sim) - 38)

print("D3P1: " + str(sum(sim_vals)))

# --- Problem 2 ---
groups: list[str] = []  # [[sack 1, sack 2, sack 3], ...]

# Group the entries by threes
for i in range(0, len(input), 3):
    groups.append(input[i : i + 3])

badges = []  # Stores the similar chars between each group (their badge)

# Find the badges
for g in groups:
    for c in list(g[0]):
        if c in list(g[1]) and c in list(g[2]):
            badges.append(c)
            break

b_vals = []  # Stores the numerical value for each character

# Convert the chars to their values
for b in badges:
    if b.islower():
        b_vals.append(ord(b) - 96)
    else:
        b_vals.append(ord(b) - 38)

print("D3P2: " + str(sum(b_vals)))
