input = None

with open("input.txt", "r") as f:
  input = f.read().split('\n')

# --- Problem 1 ---
contained = 0 # Stores the number of items fully contained in the other

for line in input:
  [r1, r2] = line.split(',') # Split each line by comma
  # If r2 in r1
  if int(r1.split('-')[0]) >= int(r2.split('-')[0]) and int(r1.split('-')[1]) <= int(r2.split('-')[1]):
    contained += 1
  # If r1 in r2
  elif int(r1.split('-')[0]) <= int(r2.split('-')[0]) and int(r1.split('-')[1]) >= int(r2.split('-')[1]):
    contained += 1

print("D4P1: " + str(contained))

# --- Problem 2 ---
# NOTE: This solution is horrendously bad in space and time complexities
pairs = [] # Stores the numbers contained in each pair

# Generate the pairs
for line in input:
  [r1, r2] = line.split(',')
  pair = (
    [x for x in range(int(r1.split('-')[0]), int(r1.split('-')[1]) + 1)],
    [y for y in range(int(r2.split('-')[0]), int(r2.split('-')[1]) + 1)]
  )
  pairs.append(pair)

overlaps = 0 # Stores the number of pairs that overlap

# Check for overlaps
for pair in pairs:
  for v in pair[0]:
    if v in pair[1]: # If there is an overlap
      overlaps += 1
      break

print("D4P2: " + str(overlaps))