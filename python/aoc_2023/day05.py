from dataclasses import dataclass, field
from typing import List


@dataclass
class Range:
    src_start: int
    src_end: int
    offset: int  # can be negative

    def get(self, i: int):
        if i >= self.src_start and i <= self.src_end:
            return i + self.offset


@dataclass
class Mapping:
    name: str
    ranges: List[Range] = field(default_factory=list)

    def update(self, s: str):
        "take an individual line like 0 15 37 and update the internal map"
        dst, src, rng = [int(i) for i in s.split()]
        r = Range(src_start=src, src_end=src + rng, offset=dst - src)
        self.ranges.append(r)

    def get(self, i: int):
        for r in self.ranges:
            if new := r.get(i):
                return new
        return i


@dataclass
class Pipeline:
    maps: List[Mapping]

    @classmethod
    def from_lines(cls, lines: List[str]):
        "Parse a text blob to create a series of mappings"
        maps = []
        m = None
        for line in lines:
            line = line.strip()
            if line.endswith("map:"):
                name = line.split()[0]
                m = Mapping(name=name)
            elif not line:
                maps.append(m)
                m = None
            else:
                if m is None:
                    raise RuntimeError(
                        "got a parseable line but no Mapping obj to update: %s", line
                    )
                m.update(line)
        maps.append(m)
        return cls(maps=maps)

    def get(self, i: int, debug=False):
        "Return the final value as i goes through each mapping in the pipeline"
        for mapping in self.maps:
            new = mapping.get(i)
            if debug:
                print(f"{i} -> {mapping.name} -> {new}")
            i = new
        return i
