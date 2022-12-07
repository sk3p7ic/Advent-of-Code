input = None

with open("input.txt", "r") as f:
    input = f.read().split("\n")

# --- Part 1 ---
class DirNode:
    """Class for directories in the file tree."""
    def __init__(self, name):
        self.name = name
        self.children = {}
        self.parent = None

    def add(self, node):
        self.children[node.name] = node
        node.parent = self

    def get_size(self):
        return sum(node.get_size() for node in self.children.values())

class FileNode:
    """Class for files in the file tree."""
    def __init__(self, name, bytes):
        self.name = name
        self.bytes = int(bytes)
        self.parent = None

    def get_size(self):
        return self.bytes

def smol_size(fs, target_size=100000):
    """Gets the directories meeting a given target size."""
    res = []
    for node in fs.children.values():
        if isinstance(node, DirNode):
            if node.get_size() <= target_size:
                res.append(node)
            res.extend(smol_size(node, target_size))
    return res

root = DirNode("/") # Create the root of the file system
curr_dir = root # Set the current directory
idx = 0 # Set the index for the line that we are parsing
input = input[1:] # Delete the first line (we've already created the '/')
# Build the file tree
while idx < len(input):
    # Get the current line, if possible
    try:
        line = input[idx]
    except IndexError:
        break
    # If we're now parsing the output of 'ls'
    if line.startswith("$ ls"):
        idx += 1
        try:
            # While the current line is not a command
            while not input[idx].startswith("$"):
                subline = input[idx] # Get this line
                if subline.startswith("dir"):
                    curr_dir.add(DirNode(subline.split(" ")[1]))
                else:
                    try:
                        bytes, name = subline.split(" ")
                        curr_dir.add(FileNode(name, bytes))
                    except:
                        print(f"[!] Error parsing: '{subline}'")
                idx += 1
        except IndexError:
            continue
    # If this line is to change the current directory
    elif line.startswith("$ cd"):
        # Goto root
        if line.startswith("$ cd /"):
            curr_dir = root
        # Go back
        elif line.startswith("$ cd ..") and curr_dir.parent != None:
            curr_dir = curr_dir.parent
        # Handle normal change directory
        elif line.startswith("$ cd"):
            cmd = line.split(' ')
            if cmd[2] not in curr_dir.children:
                curr_dir.add(DirNode(cmd[2]))
            curr_dir = curr_dir.children[cmd[2]]
        idx += 1

print("D7P1: " + str(
    sum(node.get_size() for node in smol_size(root))
    ))

# --- Part 2 ---
# Calculate the target size needed
free_space = 70000000 - root.get_size()
needed = 30000000 - free_space

def get_the_big_d(fs, target_bytes):
    """Gets the bigger directories with a given target size."""
    res = set()
    if fs.get_size() >= target_bytes:
        res.add(fs)
    for node in fs.children.values():
        if isinstance(node, DirNode):
            if node.get_size() >= target_bytes:
                res.add(node)
            res.update(get_the_big_d(node, target_bytes))
    return res

# Get the smallest from those big bois
smol_d = min(get_the_big_d(root, needed), key=lambda b: b.get_size())

print("D7P2: " + str(smol_d.get_size()))
