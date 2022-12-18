from hashlib import md5

data = ""  # Removed for compliance with AoC rules


def get_target(target: str):
    for i in range(1000000000):
        if md5((data + str(i)).encode()).hexdigest().startswith(target):
            return i


print("D04P01:", get_target("00000"))
print("D04P02:", get_target("000000"))
