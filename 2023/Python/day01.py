with open('../Rust/problems/inputs/day01.in.txt') as f:
    lines = f.readlines()

testlines_1 = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet""".splitlines()

testlines_2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen""".splitlines()


def find_digits(line: str) -> int:
    first, last = 0, 0
    for i, c in enumerate(line):
        if c.isdigit():
            first = int(c)
            break
    for i, c in enumerate(line[::-1]):
        if c.isdigit():
            last = int(c)
            break
    return first * 10 + last


def do_p1(lines: list[str]) -> int:
    return sum(map(find_digits, lines))


def do_p2(lines: list[str]) -> int:
    digits = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight',
              'nine']
    # Replace spelled out digits with numbers
    for i, d in enumerate(digits):
        ds, de = d[0], d[-1]  # first and last char of digit
        lines = list(map(lambda ln: ln.replace(d, f"{ds}{i+1}{de}"), lines))
    # Now we can just call find_digits
    return sum(map(find_digits, lines))


print("[TEST] D01P01:", do_p1(testlines_1))
print("[TEST] D01P02:", do_p2(testlines_2))
print("[2023] D01P01:", do_p1(lines))
print("[2023] D01P02:", do_p2(lines))
