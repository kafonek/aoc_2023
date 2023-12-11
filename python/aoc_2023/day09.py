from typing import List


def make_diffs(numbers: List[int]) -> List[int]:
    """
    Return a list of size one less than input numbers, is the diff between each number

    numbers = [1, 2, 3]
    assert make_diffs(numbers) == [1, 1, 1]
    """
    
    return [j - i for i, j in zip(numbers, numbers[1:])]

def make_diff_hierarchy(numbers: List[int]) -> List[List[int]]:
    """
    Return a list of lists for all diffs until len(last_diff) == 1

    numbers = [1, 1, 2, 4]
    assert make_diff_hierarchy(numbers) == [[0, 1, 2], [1, 1], [0]]
    """
    diffs = make_diffs(numbers)
    total_diffs = [diffs]
    while len(diffs) > 1:
        diffs = make_diffs(diffs)
        total_diffs.append(diffs)
    return total_diffs

def next_diffs(diff_hierarchy: List[List[int]]) -> List[int]:
    """
    Given a diff hierarchy, return a list of 'next results'

    diff_hiearchy = [[0, 1, 2], [1, 1], [0]]
    assert next_diffs(diff_hierarchy) == [3, 1]
    """
    results = []
    to_add = diff_hierarchy[-1][-1] # should always be 0 in theory
    for diff in reversed(diff_hierarchy[:-1]):
        value = diff[-1] + to_add
        results.append(value)
        to_add = value
    return results

def previous_diffs(diff_hierarchy: List[List[int]]):
    """
    Given a diff hierarchy, return a list of 'next results'

    diff_hiearchy = [[0, 1, 2], [1, 1], [0]]
    assert next_diffs(diff_hierarchy) == [-1, 1]]
    """
    results = []
    to_subtract = diff_hierarchy[-1][-1] # should always be 0 in theory
    for diff in reversed(diff_hierarchy[:-1]):
        value = diff[0] - to_subtract
        results.append(value)
        to_subtract = value
    return results
