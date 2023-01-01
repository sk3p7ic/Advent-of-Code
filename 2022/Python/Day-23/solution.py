with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")

data = """....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..""".strip().split(
    "\n"
)

elves: dict[tuple[int, int], bool] = {}

for y, row in enumerate(data):
    for x, col in enumerate(list(row)):
        if col == "#":
            elves[(x, y)] = True

ELF_COUNT = len(elves)

TRANS_MATRX: list[list[tuple[int, int]]] = [
    [(-1, 1), (0, 1), (1, 1)],  # North
    [(-1, -1), (0, -1), (1, -1)],  # South
    [(-1, 1), (-1, 0), (-1, -1)],  # West
    [(1, 1), (1, 0), (1, -1)],  # East
]


def do_check(elf: tuple[int, int], round: int) -> tuple[int, int]:
    matrix = TRANS_MATRX[round % 4]
    for dir in matrix:
        x, y = elf
        dx, dy = x + dir[0], y + dir[1]
        if (dx, dy) in elves:
            return elf
    return (elf[0] + matrix[1][0], elf[1] + matrix[1][1])


def do_round(
    elves: dict[tuple[int, int], bool], round: int
) -> dict[tuple[int, int], bool]:
    moves: dict[tuple[int, int], list[tuple[int, int]]] = {}
    for elf in elves:
        trans = do_check(elf, round)
        m = moves.get(trans, [])
        m.append(elf)
        moves[trans] = m
    for k, v in moves.items():
        if k in v:
            for e in v:
                elves[e] = True
        if len(v) == 1:
            elves[k] = True
    return elves


def find_dimens(grid: list[tuple[int, int]]) -> tuple[int, int]:
    min_x, min_y = min(grid, key=lambda t: t[0]), min(grid, key=lambda t: t[1])
    max_x, max_y = max(grid, key=lambda t: t[0]), max(grid, key=lambda t: t[1])
    return (max_x[0] - min_x[0], max_y[1] - min_y[1])


print(find_dimens(elves.keys()))
for r in range(11):
    elves = do_round(elves, r)
length, width = find_dimens(elves.keys())
print(length, width)
print((length * width) - ELF_COUNT)
