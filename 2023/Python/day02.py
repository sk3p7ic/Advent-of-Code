with open('../Rust/problems/inputs/day02.in.txt') as f:
    lines = f.readlines()

SAMPLE = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green""".splitlines()

BAG = {
    'red': 12,
    'green': 13,
    'blue': 14
}


def parse_round(round: str) -> bool:
    colors = round.split(',')
    for c in colors:
        c = c.strip().split(' ')
        if int(c[0]) > BAG[c[1]]:
            return False
    return True


def parse_game(rounds: list[list[[int, str]]]):
    min_bag = {
        'red': 0,
        'green': 0,
        'blue': 0
    }
    for r in rounds:
        count, color = r
        min_bag[color] = max(min_bag[color], count)
    return min_bag['red'] * min_bag['green'] * min_bag['blue']


def do_p1(lines: list[str]) -> int:
    plausible_games = []
    for i, line in enumerate(lines):
        rounds = line.split(':')[1].strip().split(';')
        did_break = False
        for r in rounds:
            if not parse_round(r):
                did_break = True
                break
        if not did_break:
            plausible_games.append(i + 1)
    return sum(plausible_games)


def do_p2(lines: list[str]) -> int:
    game_strs = []
    for game in lines:
        game_strs.append(game.split(':')[1].strip())
    games: list[list[tuple[int, str]]] = []
    for gg in game_strs:
        rounds = gg.split(';')  # Keeping this variable for readability
        draw_lst = [r.strip().split(',') for r in rounds]
        draws = []
        # Parse each of the draws made in a round
        for draw in draw_lst:
            # Parse each draw to tuple and save
            for cube_str in draw:
                cubes = cube_str.strip().split(' ')
                draws.append((int(cubes[0]), cubes[1]))
        games.append(draws)
    return sum(map(parse_game, games))


print("[TEST] D01P01:", do_p1(SAMPLE))
print("[TEST] D01P02:", do_p2(SAMPLE))
print("[2023] D01P01:", do_p1(lines))
print("[2023] D01P02:", do_p2(lines))
