with open('../Rust/problems/inputs/day02.in.txt') as f:
    lines = f.readlines()

SAMPLE = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green""".splitlines()


def prepare(lines: list[str]) -> list[list[str]]:
    def prepare_line(s: str) -> list[str]:
        return s.split(':')[1].strip().replace(',', '').replace(';', '')\
            .split(' ')
    games = list(map(prepare_line, lines))
    for g in games:  # Verify list is of proper form for algo
        assert len(g) % 2 == 0
    return games


def do_p1(lines: list[str]) -> int:
    max_bag = {'red': 12, 'green': 13, 'blue': 14}
    plausible_games = []
    games = prepare(lines)
    for game_num, game in enumerate(games):
        did_break = False
        for i in range(0, len(game), 2):
            n, c = int(game[i]), game[i + 1]  # Get the number and color
            if n > max_bag[c]:
                did_break = True
                break
        if not did_break:
            plausible_games.append(game_num + 1)
    return sum(plausible_games)


def do_p2(lines: list[str]) -> int:
    games = prepare(lines)
    powers = []
    for game in games:
        min_bag = {'red': 0, 'green': 0, 'blue': 0}
        for i in range(0, len(game), 2):
            n, c = int(game[i]), game[i + 1]  # Get the number and color
            min_bag[c] = max(min_bag[c], n)
        powers.append(min_bag['red'] * min_bag['green'] * min_bag['blue'])
    return sum(powers)


print("[TEST] D01P01:", do_p1(SAMPLE))
print("[TEST] D01P02:", do_p2(SAMPLE))
print("[2023] D01P01:", do_p1(lines))
print("[2023] D01P02:", do_p2(lines))
