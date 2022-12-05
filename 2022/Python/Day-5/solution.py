input = None

with open("input.txt", "r") as f:
    input = f.read().split("\n")

# -- Part 1 ---
# Find the number of stacks that there are
num_stacks = 0
stack_def_end_idx = 0
for i, line in enumerate(input):
    if "]" not in line:  # Is this the line with the stack numbers?
        num_stacks = int(line.strip().split(" ")[-1])
        stack_def_end_idx = i
        break

stacks = [[] for _ in range(int(num_stacks))]  # Build the array of stacks

# Build the stacks from the input
for i in range(stack_def_end_idx):
    line = input[i]
    for n in range(num_stacks):
        char = line[4 * n + 1]
        if char != " ":
            stacks[n].append(char)

for stack in stacks:
    stack.reverse()

# Get the move instructions from the input
instructions = []  # [(move count, source col, dest col), ...]
for line in input:
    if "move" in line:
        parts = line.split(" ")
        instr = (int(parts[1]), int(parts[3]), int(parts[5]))
        instructions.append(instr)

# Perform the move operations
for move in instructions:
    for _ in range(move[0]):
        stacks[move[2] - 1].append(stacks[move[1] - 1].pop())

# Build the output
p1_out = ""
for s in stacks:
    # Code
    p1_out += s[-1]

print("D5P1: " + p1_out)

# --- Part 2 ---

# Rebuild the stacks
stacks = [[] for _ in range(int(num_stacks))]

for i in range(stack_def_end_idx):
    line = input[i]
    for n in range(num_stacks):
        char = line[4 * n + 1]
        if char != " ":
            stacks[n].append(char)

for stack in stacks:
    stack.reverse()

# Perform the move operations
for move in instructions:
    block = []
    for _ in range(move[0]):
        block.append(stacks[move[1] - 1].pop())
    block.reverse()
    stacks[move[2] - 1].extend(block)

# Build the output
p2_out = ""
for s in stacks:
    # Code
    p2_out += s[-1]
print("D5P2: " + p2_out)
