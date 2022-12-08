input = None

with open("input.txt", "r") as f:
    input = f.read().split("\n")
    del input[-1]

# Convert the input into list of list of ints
input = list(map(lambda r: list(map(lambda t: int(t), list(r))), input))

# --- Part 1 ---
num_visible = 0  # Stores the total number of trees that are visible

for y in range(len(input)):
    for x in range(len(input[0])):
        tree = input[y][x]  # This tree, stored for convienence
        # Check left
        left = input[y][:x]
        # If this is the far left or there is a leftward tree that is taller
        if not left or (left and max(left) < tree):
            num_visible += 1
            continue
        # Check right
        right = input[y][x + 1 :]
        if not right or (right and max(right) < tree):
            num_visible += 1
            continue
        # Check up
        up = [input[dy][x] for dy in range(y)]
        if not up or (up and max(up) < tree):
            num_visible += 1
            continue
        # Check down
        down = [input[dy][x] for dy in range(y + 1, len(input))]
        if not down or (down and max(down) < tree):
            num_visible += 1
            continue

print("D8P1: " + str(num_visible))

# --- Part 2 ---
max_score = 0  # Stores the max score found for the trees

for y in range(len(input)):
    for x in range(len(input[0])):
        tree = input[y][x]  # This tree, stored for convienence
        # Check left
        score_left = 0
        left = reversed(input[y][:x])  # Reverse b/c searching to left of tree
        if left:
            for t in left:
                score_left += 1
                if t >= tree:
                    break
        # Check right
        score_right = 0
        right = input[y][x + 1 :]  # Normal b/c searching to right
        if right:
            for t in right:
                score_right += 1
                if t >= tree:
                    break
        # Check up
        score_up = 0
        up = reversed([input[dy][x] for dy in range(y)])
        if up:
            for t in up:
                score_up += 1
                if t >= tree:
                    break
        # Check down
        score_down = 0
        down = [input[dy][x] for dy in range(y + 1, len(input))]
        if down:
            for t in down:
                score_down += 1
                if t >= tree:
                    break
        # Get the total score for this tree
        score_tree = score_left * score_right * score_up * score_down
        if score_tree > max_score:
            max_score = score_tree

print("D8P2: " + str(max_score))
