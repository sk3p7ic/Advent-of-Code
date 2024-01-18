from dataclasses import dataclass
from typing import Any

from tqdm import tqdm

SAMPLE = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""


@dataclass
class Mapping:
    src: int
    dst: int
    len: int

    def apply(self, n: int) -> int | None:
        if self.src <= n < (self.src + self.len):
            offset = n - self.src
            return self.dst + offset
        else:
            return None


@dataclass
class DefaultAlmanac:
    seeds: Any
    blocks: dict[str, list[Mapping]]

    def forward(self, layer: str, on: list[int]) -> list[int]:
        maps = self.blocks.get(layer)
        results = []
        for n in tqdm(on, leave=False, desc='Forwarding Seeds'):
            res = None
            for m in maps:
                if (tmp := m.apply(n)) is not None:
                    res = tmp
                    break
            if res is None:
                res = n
            results.append(res)
        return results


@dataclass
class Almanac(DefaultAlmanac):
    seeds: list[int]
    blocks: dict[str, list[Mapping]]


@dataclass
class RangedAlmanac(DefaultAlmanac):
    seeds: list[range]
    blocks: dict[str, list[Mapping]]


def parse_almanac(almanac: str, ranged=False) -> DefaultAlmanac:
    blocks = almanac.strip().split('\n\n')
    first, rest = blocks[0], blocks[1:]
    seed_ns: list[int] = list(map(int, first.split(': ')[1].split(' ')))
    seeds = seed_ns if not ranged else []
    if ranged:
        for i in range(0, len(seed_ns), 2):
            seeds.append(range(seed_ns[i], seed_ns[i] + seed_ns[i+1]))
    maps = {}
    for block in rest:
        label, map_strs = block.splitlines()[0], block.splitlines()[1:]
        label = label.replace(' map:', '')
        mappings = []
        for ms in map_strs:
            dst, src, span = ms.split(' ')
            mappings.append(Mapping(int(src), int(dst), int(span)))
        maps[label] = mappings
    return Almanac(seeds, maps) if not ranged else RangedAlmanac(seeds, maps)


def process_p1(s: str) -> int:
    a: Almanac = parse_almanac(s)
    labels = list(a.blocks.keys())
    soils = a.forward(labels[0], a.seeds)
    ferts = a.forward(labels[1], soils)
    watrs = a.forward(labels[2], ferts)
    lghts = a.forward(labels[3], watrs)
    temps = a.forward(labels[4], lghts)
    humds = a.forward(labels[5], temps)
    locns = a.forward(labels[6], humds)
    return min(locns)


def process_p2(s: str) -> int:
    a: RangedAlmanac = parse_almanac(s, True)
    labels = list(a.blocks.keys())
    locations = []
    for r in tqdm(a.seeds, leave=False, desc='Iterating over ranges'):
        with tqdm(total=7, leave=False, desc='Applying mapping layers') as t:
            seeds = list(r)
            soils = a.forward(labels[0], seeds)
            t.update()
            ferts = a.forward(labels[1], soils)
            t.update()
            watrs = a.forward(labels[2], ferts)
            t.update()
            lghts = a.forward(labels[3], watrs)
            t.update()
            temps = a.forward(labels[4], lghts)
            t.update()
            humds = a.forward(labels[5], temps)
            t.update()
            locns = a.forward(labels[6], humds)
            t.update()
            locations.append(min(locns))
    return min(locations)


with open('../Rust/problems/inputs/day05.in.txt') as f:
    lines = f.read()

print(process_p1(SAMPLE))
print(process_p2(SAMPLE))
print("[2023] D05P01:", process_p1(lines))
# print("[2023] D05P02:", process_p2(lines))
