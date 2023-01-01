# Read the input
input = None

with open("input.txt", "r") as f:
    input = f.read().split("\n\n")

# Start working on problems
elves = []  # Stores the totals for each elf

# Problem 1
for elf in input:
    cals = elf.split("\n")  # Get the list of calories for each elf
    elves.append(
        sum(map(lambda c: int(c), cals))
    )  # Convert strings to ints, sum, and append to parent list

elves.sort(reverse=True)  # Sort the list with largest vals at the front

print(f"D1P1: {elves[0]}")

# Problem 2
top_3 = sum(elves[:3])  # Get the top 3 totals and sum them

print(f"D1P2: {top_3}")
