with open("input.txt", "r") as f:
    data = f.read().strip().split("\n")

# --- Part 1 ---
nice_bois = 0
for s in data:
    if "ab" in s or "cd" in s or "pq" in s or "xy" in s:
        continue
    vowels = []
    double_double_with_cheese = False
    for i, c in enumerate(list(s)):
        if i > 0 and not double_double_with_cheese:
            if c == s[i - 1]:
                double_double_with_cheese = True
        if c in ["a", "e", "i", "o", "u"]:
            vowels.append(c)
    if len(vowels) >= 3 and double_double_with_cheese:
        nice_bois += 1
print("D05P01:", nice_bois)

# --- Part 2 ---
import subprocess

# Regex makes this insanely easier
cmd = r'cat input.txt |  grep "\(..\).*\1" | grep "\(.\).\1" | wc -l'
nice_as_rice = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE).stdout.read()
print("D05P02:", nice_as_rice.decode().strip())
