from collections import deque

with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")


# [[x, -y]] list of index translations
TRNS_MTRX = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [0, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
]

# The grid of the elves and open positions
grid = deque(map(deque, data))


def add(dir: str) -> None:
    """
    Adds a row or column to the grid in a given direction.

    Parameters
        dir : The direction to add a column or row to, either {`n`, `s`, `e`,
        or `w`}.
    """
    match dir:
        case "n":
            grid.appendleft(deque(["." for _ in range(len(grid[0]))]))
        case "s":
            grid.append(deque(["." for _ in range(len(grid[0]))]))
        case "w":
            for g in grid:
                g.appendleft(".")
        case "e":
            for g in grid:
                g.append(".")


def check_for_direction(dir: str, elf: tuple[int, int]) -> bool:
    """
    Checks if an elf may move in a given direction.

    Parameters
        dir : The direction to check, either {`n`, `s`, `e`, or `w`}.
        elf : The elf's position to check relative to, in the form `(x, y)`.

    Returns
        bool : Whether or not the elf may move in the given direction.
    """
    # Get the needed translation vector for this direction
    match dir:
        case "n":
            vecs = TRNS_MTRX[:3]
        case "e":
            vecs = [TRNS_MTRX[2], TRNS_MTRX[5], TRNS_MTRX[8]]
        case "s":
            vecs = TRNS_MTRX[-3:]
        case "w":
            vecs = [TRNS_MTRX[0], TRNS_MTRX[3], TRNS_MTRX[6]]
        case _:
            return False
    # Check if the move may occur
    for v in vecs:
        dx, dy = elf[0] + v[0], elf[1] + v[1]
        # If the move should happen to a coord off-grid
        if dx < 0 or dx == len(grid[0]) or dy < 0 or dy == len(grid):
            add(dir)  # Add the needed column or row
            continue  # Skip since this col / row will be free to move into
        # If the position the elf would like to move into is taken
        if grid[dy][dx] != ".":
            return False
    return True


def get_move(
    elf: tuple[int, int], round: int
) -> tuple[tuple[int, int], tuple[int, int]]:
    """
    Gets the move that an elf needs to perform.

    Parameters:
        elf : The elf's current position, in the form `(x, y)`.
        round : The number round that is currently being simulated.

    Returns:
        elf : The elf's coordinates that were passed into this method.
        delta_elf : The change in the elf's position that may take place.
    """
    # Stores the orders of directions that the elves will check
    directions = [
        ["n", "s", "w", "e"],
        ["s", "w", "e", "n"],
        ["w", "e", "n", "s"],
        ["e", "n", "s", "w"],
    ]
    for dir in directions[round % 4]:
        # If the direction is clear for the elf to move to
        if check_for_direction(dir, elf):
            # Get the delta values for the elf's position in the grid
            match dir:
                case "n":
                    dx, dy = 0, -1
                case "s":
                    dx, dy = 0, 1
                case "e":
                    dx, dy = 1, 0
                case "w":
                    dx, dy = -1, 0
                case _:
                    continue
            return (elf, (dx, dy))
    return (elf, (0, 0))


def do_round(round: int) -> None:
    """Simulates a round."""
    # Stores the moves that the elves would like to make
    moves = []
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] == ".":
                continue
            elf_pos = (x, y)
            moves.append(get_move(elf_pos, round))
    new_positions = {}
    # Calculate the positions that the elves are attempting to navigate to
    for m in moves:
        this, delf = m  # This elf, change in elf direction
        x, y = this
        dx, dy = delf
        px, py = x + dx, y + dy
        new_positions[(px, py)] = new_positions.get((px, py), 0) + 1
    # Perform the moves
    for m in moves:
        this, delf = m  # This elf, change in elf direction
        x, y = this
        dx, dy = delf
        px, py = x + dx, y + dy
        # If two or more elves would try to go to the same position
        if new_positions[(px, py)] > 1:
            continue
        grid[py][px] = "#"
        grid[y][x] = "."


def count_open() -> int:
    """Counts the number of open positions in the grid and returns it."""
    open_spots = 0
    for row in grid:
        for spot in row:
            if spot == ".":
                open_spots += 1
    return open_spots


for n in range(10):
    do_round(n)
print(count_open())
