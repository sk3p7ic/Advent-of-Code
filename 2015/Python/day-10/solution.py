data = "1113122113"


def do_round(data: str) -> str:
    counts = []
    ptr = 0
    while ptr < len(data):
        n, this = data[ptr], 0
        while ptr < len(data) and data[ptr] == n:
            this += 1
            ptr += 1
        counts.append((n, this))
    new_string = ""
    for (n, occurrences) in counts:
        new_string += str(occurrences) + str(n)
    return new_string


for _ in range(40):
    data = do_round(data)
print("Day 10, Part 01:", len(data))

data = "1113122113"
for _ in range(50):
    data = do_round(data)
print("Day 10, Part 02:", len(data))
