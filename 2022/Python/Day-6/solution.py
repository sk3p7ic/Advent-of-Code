input = None

with open("input.txt", "r") as f:
    input = f.read()

# A sliding solution would've been more efficient but I was having issues with
# that, presumably because of the way that slicing a string works, which I did
# not initially remember.

# --- Part 1 ---
idx = 0  # Stores the index for message start
for i in range(len(input) - 4):
    four = input[i : i + 4]  # Get the next four characters
    repeat = False  # Stores if there was a repreated character
    chars = {}  # Used for easier lookup of the next four characters
    for c in list(four):
        if c in chars.keys():
            repeat = True
            break
        chars[c] = 0
    if not repeat:  # If the next four are good
        idx = i + 4
        break

print("D6P1: " + str(idx))

# --- Part 2 ---
idy = 0
for i in range(len(input) - 14):
    fourteen = input[i : i + 14]
    repeat = False
    chars = {}
    for c in list(fourteen):
        if c in chars.keys():
            repeat = True
            break
        chars[c] = 0
    if not repeat:
        idy = i + 14
        break

print("D6P2: " + str(idy))
