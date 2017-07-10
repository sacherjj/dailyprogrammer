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


methods = [zero_comb, zero_sum, zero_optimal]

with open('run_time.txt', 'w') as f:
    f.write('n comb loop quad\n')

cnt_limit = 5
for n in range(10, 300):
    print('Evaluating {}'.format(n))
    for cnt in range(5):
        vals = [choice([i for i in range(-100, 100)]) for r in range(n)]
        run_data = [n, 0, 0, 0]
        for i, method in enumerate(methods):
            num_list = [x for x in vals]
            start = time.time()
            solution = method(num_list)
            dur = time.time() - start
            run_data[i+1] += dur
    # calc average
    for i in range(1, 4):
        run_data[i] /= cnt_limit
    print(run_data)
    with open('run_time.txt', 'a') as f:
        f.write(' '.join([str(x) for x in run_data]))
        f.write('\n')
