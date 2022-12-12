# This one was pretty annoying and I unfortunately had to seek help.
# Credit: https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/12.py
from collections import deque

with open("input.txt", "r") as f:
    input = f.read().strip().split("\n")

# --- Part 1 ---
str_board: list[list[str]] = list(map(lambda line: list(line), input))
board = [[0 for _ in range(len(r))] for r in str_board]

start_pos: tuple[int, int] = (0, 0)

for y, line in enumerate(str_board):
    for x, char in enumerate(line):
        if char == "S":
            start_pos = (x, y)
            # Convert to the appropriate character value
            board[y][x] = 1
        elif char == "E":
            # Convert to the appropriate character value
            board[y][x] = 26
        else:
            # Convert to the appropriate character value
            board[y][x] = ord(str_board[y][x]) - ord("a") + 1


def bfs(path_queue: deque):
    visited_positions = set()
    while path_queue:
        (x, y), steps = path_queue.popleft()
        if (x, y) in visited_positions:
            continue
        visited_positions.add((x, y))
        if str_board[y][x] == "E":
            return steps
        # Check in the four directions
        for mod_y, mod_x in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
            dy = y + mod_y
            dx = x + mod_x
            if (
                0 <= dy < len(board)
                and 0 <= dx < len(board[0])
                and board[dy][dx] <= 1 + board[y][x]
            ):
                path_queue.append(((dx, dy), steps + 1))


path_queue_p1 = deque()
for y, line in enumerate(board):
    for x, _ in enumerate(line):
        if (x, y) == start_pos:
            path_queue_p1.append(((x, y), 0))
print("D12P1:", bfs(path_queue_p1))

# --- Part 2 ---
path_queue_p2 = deque()
for y, line in enumerate(board):
    for x, height in enumerate(line):
        if height == 1:
            path_queue_p2.append(((x, y), 0))
print("D12P2:", bfs(path_queue_p2))
