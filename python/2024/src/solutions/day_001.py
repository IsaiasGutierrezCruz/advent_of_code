from aocd.models import Puzzle

def format_string_to_lists(input_string):
    list1 = []
    list2 = []
    
    lines = input_string.strip().split('\n')
    for line in lines:
        num1, num2 = line.split()
        list1.append(int(num1))
        list2.append(int(num2))
    
    return list1, list2


def solve_part_01(input_string):
    list1, list2 = format_string_to_lists(input_string)

    differences = []
    for left_item, right_item in zip(sorted(list1), sorted(list2)):
        differences.append(abs(left_item - right_item))
    return sum(differences)




def day_001(input_data: str) -> None: 
    print("Day 1. Part 1: ", solve_part_01(input_data))