input = None

with open("input.txt", "r") as f:
    input = f.read().split("\n")

del input[-1]

# --- Part 1 ---
def get_strength_at_cycle_number(cycle: int, x_vals: list[int]) -> int:
    """Calculates the signal strength at a given cycle."""
    return cycle * x_vals[cycle - 1]


x_over_cycles: list[int] = [1]  # Stores the values of the x register at each cycle

for line in input:
    curr_x = x_over_cycles[-1]
    if line == "noop":
        x_over_cycles.append(curr_x)
    else:
        n = int(line.split(" ")[1])
        x_over_cycles.extend([curr_x, curr_x + n])

# Calulate the strengths at the target cycle values
target_cycles = [
    get_strength_at_cycle_number(t, x_over_cycles) for t in [20, 60, 100, 140, 180, 220]
]

print("D10P1:", sum(target_cycles))

# --- Part 2 ---
crt: list[str] = []


def render_char(value: int, line: str) -> str:
    """Renders the appropriate character to the given line."""
    if len(line) in [value - 1, value, value + 1]:
        line += "#"
    else:
        line += "."
    return line


def check_line(line: str, crt: list[str]) -> bool:
    """Checks if a line has met the line length."""
    if len(line) == 40:
        crt.append(line)
        return True
    return False


current_line = ""  # The current line
x_reg = 1  # The value of the x register
for line in input:
    if line == "noop":
        current_line = render_char(x_reg, current_line)
        current_line = "" if check_line(current_line, crt) else current_line
    else:
        # For both render cycles the value should not change
        current_line = render_char(x_reg, current_line)
        current_line = "" if check_line(current_line, crt) else current_line
        current_line = render_char(x_reg, current_line)
        current_line = "" if check_line(current_line, crt) else current_line
        # Now I can apply the add operation
        x_reg += int(line.split(" ")[1])
crt.append(current_line)

print("D10P2:")
for line in crt:
    print(line)
