# This one also had me stumped and I used someone else's code as a reference
# again. It's all a good learning opportunity at this point.
# Credit: https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/13.py
from functools import cmp_to_key
import json

input = None
with open("input.txt", "r") as f:
    input = f.read().strip().split("\n\n")

# --- Part 1 ---
parsed = list(
    map(lambda group: tuple(map(lambda s: json.loads(s), group.split("\n"))), input)
)


def process_p1(lhs, rhs):
    """Processes the inputs to determine if the sides are in the correct order."""
    if isinstance(lhs, int) and isinstance(rhs, int):
        if lhs < rhs:
            return 1
        elif lhs == rhs:
            return 0
        else:
            return -1
    elif isinstance(lhs, list) and isinstance(rhs, list):
        i = 0
        while i < len(lhs) and i < len(rhs):
            subvalue = process_p1(lhs[i], rhs[i])
            if subvalue == -1 or subvalue == 1:
                return subvalue
            i += 1
        if i == len(lhs) and i < len(rhs):
            return 1
        elif i < len(lhs) and i == len(rhs):
            return -1
        else:
            return 0
    elif isinstance(lhs, int) and isinstance(rhs, list):
        return process_p1([lhs], rhs)
    else:
        return process_p1(lhs, [rhs])


results = []

for pair in parsed:
    lhs, rhs = pair
    results.append(process_p1(lhs, rhs))
num_correct = 0
for i, res in enumerate(results):
    if res == 1:
        num_correct += i + 1
print("D13P1:", num_correct)

# --- Part 2 ---
packets = []
for pair in parsed:
    packets.extend([pair[0], pair[1]])
packets.append([[2]])
packets.append([[6]])

packets = sorted(packets, key=cmp_to_key(lambda lhs, rhs: process_p1(rhs, lhs)))
decoder_key = 1
for i, packet in enumerate(packets):
    if packet == [[2]] or packet == [[6]]:
        decoder_key *= i + 1
print(decoder_key)
