input = None

with open("input.txt", "r") as f:
    input = f.read().strip().split("\n")


# --- Part 1 ---
# Store the positions of the head and tail
pos_head = {"x": 0, "y": 0}
pos_tail = {"x": 0, "y": 0}

visited_tail_positions = set()  # Stores set of tuples for visited tail positions


def is_head_attached(pos_head, pos_tail):
    # Define these vars, for convienence in comparisons
    tail_x = pos_tail["x"]
    tail_y = pos_tail["y"]
    head_tuple = (pos_head["x"], pos_head["y"])  # Create tuple for comparisons
    # Check for overlap
    if (tail_x, tail_y) == head_tuple:
        return True
    # Check left
    if (tail_x - 1, tail_y) == head_tuple:
        return True
    # Check right
    if (tail_x + 1, tail_y) == head_tuple:
        return True
    # Check up
    if (tail_x, tail_y + 1) == head_tuple:
        return True
    # Check down
    if (tail_x, tail_y - 1) == head_tuple:
        return True
    # Check up-left and up-right diagonals
    if (tail_x - 1, tail_y + 1) == head_tuple or (tail_x + 1, tail_y + 1) == head_tuple:
        return True
    # Check down-left and down-right diagonals
    if (tail_x - 1, tail_y - 1) == head_tuple or (tail_x + 1, tail_y - 1) == head_tuple:
        return True
    return False


def move(direction: str, amount: int, pos_head, pos_tail):
    """Handles moving a rope with only a head and a tail in a given direction."""
    for _ in range(amount):
        # Store the last position of the head
        last_head_pos = {"x": pos_head["x"], "y": pos_head["y"]}
        # Move left
        if direction == "L":
            pos_head["x"] -= 1
        if direction == "R":
            pos_head["x"] += 1
        if direction == "U":
            pos_head["y"] += 1
        if direction == "D":
            pos_head["y"] -= 1

        # Check if this move unlinked the tail and update if so
        if not is_head_attached(pos_head, pos_tail):
            pos_tail = last_head_pos
        visited_tail_positions.add((pos_tail["x"], pos_tail["y"]))
    return pos_head, pos_tail


for line in input:
    direction, amount = line.split(" ")
    # I had to use all these params b/c I was having weird memory issues
    pos_head, pos_tail = move(direction, int(amount), pos_head, pos_tail)
print("D9P1:", len(visited_tail_positions))

# -- Part 2 ---
# Create a rope of the knots and their positions
rope = [{"x": 0, "y": 0} for _ in range(10)]
# Reset the set of visited tail positions
visited_tail_positions = set()


def move_bigger_boi(direction: str, amount: int, rope):
    """Handles moving a rope that may have more than one knot in it."""
    # Move the head first
    for _ in range(amount):
        head = rope[0]
        if direction == "L":
            head["x"] -= 1
        if direction == "R":
            head["x"] += 1
        if direction == "U":
            head["y"] += 1
        if direction == "D":
            head["y"] -= 1
        rope[0] = head

        # Shift the rest of the list
        for i in range(1, len(rope)):
            knot = rope[i]  # This segment
            prev = rope[i - 1]  # The previous segment

            # If the segment is disconnected from the rest of the rope
            if not is_head_attached(prev, knot):
                # Shift this knot towards prev
                if prev["x"] > knot["x"]:
                    knot["x"] += 1
                elif prev["x"] < knot["x"]:
                    knot["x"] -= 1
                if prev["y"] > knot["y"]:
                    knot["y"] += 1
                elif prev["y"] < knot["y"]:
                    knot["y"] -= 1
            rope[i] = knot
        tail = rope[-1]
        visited_tail_positions.add((tail["x"], tail["y"]))


for line in input:
    direction, amount = line.split(" ")
    move_bigger_boi(direction, int(amount), rope)

print("D9P2:", len(visited_tail_positions))
