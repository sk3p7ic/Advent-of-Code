input = None

with open("input.txt", "r") as f:
    input = f.read().strip().split("\n\n")


# --- Part 1 ---
class Operation:
    def __init__(self, opr: str, compare_to: str):
        """Creates a new Operation object.
        Params:
            opr (str): The operation character to use.
            compare_to (str): The value that should be used for the right-hand side of an operation."""
        self.compare_to = compare_to
        # Create the functions for running the relevant operation
        if opr == "+":
            self.do = lambda lhs, rhs: lhs + rhs
        elif opr == "-":
            self.do = lambda lhs, rhs: lhs - rhs
        elif opr == "*":
            self.do = lambda lhs, rhs: lhs * rhs
        else:
            self.do = lambda lhs, rhs: lhs / rhs


class TestCase:
    def __init__(self, div_target: int, true_monkey: int, false_monkey: int):
        """Creates a new TestCase object.
        Params:
            div_target (int): The target value that will be used to determine if a worry level can be divided by this given target value.
            true_monkey (int): The number of the monkey that will be returned if `v % div_target` = 0 for a given int value v.
            false_monkey (int): The number of the monkey that will be returned if `v % div_target` != 0 for a given int value v."""
        self.div_target = div_target
        self.prep = lambda v: v // 3
        self.do = lambda v: true_monkey if v % div_target == 0 else false_monkey


class Monkey:
    def __init__(
        self, monkey_number: int, items: list[int], operation: Operation, test: TestCase
    ):
        self.monkey_number = monkey_number
        self.items = items
        self.operation = operation
        self.test = test
        self.inspections = 0

    def do_round(self, lcm: int, do_worry: bool) -> list[tuple[int, int]]:
        moves: list[tuple[int, int]] = []
        while len(self.items) > 0:
            item = self.items.pop()  # Get the next item
            # Determine what will be the right hand side of its operation
            rhs = (
                item
                if self.operation.compare_to == "old"
                else int(self.operation.compare_to)
            )
            score = self.operation.do(item, rhs)
            if do_worry:
                # Integer division by 3
                score = self.test.prep(score)
            # Get which monkey to toss this item to
            target = self.test.do(score)
            # Add this item, with its score reduced, to the list of moves that need to be made
            moves.append((target, score % lcm))
            self.inspections += 1
        return moves


class MonkeyManager:
    @staticmethod
    def create_monkey(monkey_number: int, lines: list[str]) -> Monkey:
        """Creates a new Monkey object from a given block of text describing the monkey."""
        items = list(map(lambda n: int(n.strip()), lines[1].split(":")[1].split(",")))
        opr_line = lines[2].split("=")[1].strip().split(" ")
        operation = Operation(opr_line[1], opr_line[2])
        test_target = int(lines[3].split(" ")[-1])
        true_monkey = int(lines[4].split(" ")[-1])
        false_monkey = int(lines[5].split(" ")[-1])
        test_case = TestCase(test_target, true_monkey, false_monkey)
        return Monkey(monkey_number, items, operation, test_case)

    @staticmethod
    def calc_monkey_business(counts: list[int]) -> int:
        if len(counts) == 0:
            return 0
        mb = 1
        for c in counts:
            mb *= c
        return mb

    def __init__(self, monkeys: list[Monkey]):
        self.monkeys = monkeys
        # Calculate a least common multiple that can be used to knock down the
        # values being calculated for each monkey. This helps immensely with
        # performance.
        self.least_common_multiple = 1
        for monkey in self.monkeys:
            self.least_common_multiple *= monkey.test.div_target

    def do_round(self, do_worry=True):
        for monkey in self.monkeys:
            moves = monkey.do_round(self.least_common_multiple, do_worry)
            for (target, score) in moves:
                self.transfer(target, score)

    def transfer(self, target: int, item: int):
        self.monkeys[target].items.append(item)

    def find_highest_num_inspections(self, num_monkeys=2) -> list[int]:
        counts = []
        for monkey in self.monkeys:
            counts.append(monkey.inspections)
        counts.sort(reverse=True)
        return counts[:num_monkeys]


monkeys = []
# Build the list of monkeys and their manager
for i, block in enumerate(input):
    monkeys.append(MonkeyManager.create_monkey(i, block.strip().split("\n")))
manager = MonkeyManager(monkeys)


# Run the rounds
for round in range(20):
    manager.do_round()

# Get the most active monkeys and display their scores
counts = manager.find_highest_num_inspections()
print("D11P1:", manager.calc_monkey_business(counts))

# --- Part 2 ---

# Rebuild the list of monkeys
monkeys = []
for i, block in enumerate(input):
    monkeys.append(MonkeyManager.create_monkey(i, block.strip().split("\n")))
manager = MonkeyManager(monkeys)


for round in range(10000):
    manager.do_round(False)

counts = manager.find_highest_num_inspections()
print("D11P2:", manager.calc_monkey_business(counts))
