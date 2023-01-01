with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")

data = """1
2
-3
3
-2
0
4""".split(
    "\n"
)


# Convert the list into a list of ints
data = list(map(int, data))

d_data = list(enumerate(data))

for i, v in enumerate(data):
    idx = 0
    while idx < len(d_data):
        elem = d_data[idx]
        if elem[0] != i:
            idx += 1
            continue
        break
    d_idx = idx + v
    elem = d_data.pop(idx)
    if d_idx == 0:
        d_data.append(elem)
    else:
        d_data.insert(d_idx, elem)
    print(v, d_data, idx, d_idx)
