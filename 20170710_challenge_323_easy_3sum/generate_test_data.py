from random import choice


with open('test_data_large.txt', 'w') as f:
    for i in range(5):
        f.write(' '.join([str(choice([i for i in range(-100, 100)]))
                          for r in range(1000)]))
        f.write('\n')
