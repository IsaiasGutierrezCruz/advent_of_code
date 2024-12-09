from aocd.models import Puzzle
from solutions.day_001 import day_001

YEAR = 2024

def main() -> None:
    puzzle = Puzzle(year=YEAR, day=1)
    # print("Puzzle data: ", puzzle.input_data)
    # print(type(puzzle.input_data))
    day_001(puzzle.input_data)
    # print("Part 1: ", solve_part_01(puzzle.input_data))
