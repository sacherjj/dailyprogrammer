import time
from random import choice
from itertools import combinations


def zero_optimal(num_list):
    """
    Quadratic algorithm from

    https://en.wikipedia.org/wiki/3SUM
    """
    output = []
    num_list.sort()
    length = len(num_list)
    for i in range(length-2):
        a = num_list[i]
        start = i + 1
        end = length - 1
        while start < end:
            b = num_list[start]
            c = num_list[end]
            if a + b + c == 0:
                output.append((a, b, c))
                end -= 1
            elif a + b + c > 0:
                end -= 1
            else:
                start += 1


def zero_sum(num_list):
    """
    My initial solution
    """
    num_list.sort()
    solution = set()
    for i, val_i in enumerate(num_list[:-2]):
        for j in range(i+1, len(num_list) - 1):
            val_j = num_list[j]
            for k in range(j+1, len(num_list)):
                val_k = num_list[k]
                if val_i + val_j + val_k == 0:
                    solution.add((val_i, val_j, val_k))
    return solution


def zero_comb(num_list):
    """
    Another solution in Thread
    """
    return {tuple(sorted(n)) for n in combinations(num_list, 3) if sum(n) == 0}

inputs = []

with open('../test_data_large.txt', 'r') as f:
    for line in f.readlines():
        inputs.append(line.rstrip('\n'))

methods = [('itertools', zero_comb), ('looping', zero_sum), ('quadratic', zero_optimal)]

for vals in inputs:
    print('Evaluating {}'.format(vals))
    for method in methods:
        method_name, method_obj = method
        num_list = [int(x) for x in vals.split(' ')]
        start = time.time()
        solution = method_obj(num_list)
        print('Time: {} for {}'.format(time.time()-start, method_name))
 #       print(solution)
    print('---')

