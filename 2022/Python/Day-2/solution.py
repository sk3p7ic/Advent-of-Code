input = None

with open("input.txt", "r") as f:
  input = f.read().split('\n')

# Problem 1
shapes = {
  'A': 1,
  'B': 2,
  'C': 3,
  'X': 1,
  'Y': 2,
  'Z': 3
}

total_score = 0

for round in input:
  o, s = round.split(' ') # Opponent, self
  if shapes[o] > shapes[s]: # Opponent may win
    # Check for Rock beats Scissors
    if shapes[o] == 3 and shapes[s] == 1: total_score += 7
    else: total_score += shapes[s]
  elif shapes[o] < shapes[s]: # Self may win
    # Check for Rock beats Scissors
    if shapes[o] == 1 and shapes[s] == 3: total_score += 3
    else: total_score += shapes[s] + 6
  else: # Draw
    total_score += shapes[s] + 3

print(f"D2P1: {total_score}")

# Problem 2
shapes = {
  'A': 1,
  'B': 2,
  'C': 3,
}

total_score = 0

for round in input:
  opp, out = round.split(' ')
  if out == 'X':
    if shapes[opp] == 1:
      total_score += 3
    if shapes[opp] == 2:
      total_score += 1
    if shapes[opp] == 3:
      total_score += 2
  elif out == 'Z':
    if shapes[opp] == 1:
      total_score += 8
    if shapes[opp] == 2:
      total_score += 9
    if shapes[opp] == 3:
      total_score += 7
  else:
    total_score += shapes[opp] + 3

print(f"D2P2: {total_score}")