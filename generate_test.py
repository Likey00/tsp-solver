from random import randint
from math import sqrt
import sys

if len(sys.argv) == 1:
    print('Please enter the number of vertices to generate')
    exit()

n = int(sys.argv[1])
random_coords = [(randint(1, 1000), randint(1, 1000)) for _ in range(n)]

with open('test.txt', 'w') as f:
    for i in range(n):
        for j in range(n):
            f.write(str(round(sqrt((random_coords[i][0] - random_coords[j][0])**2 + (random_coords[i][1] - random_coords[j][1])**2))))
            f.write(' ')
        f.write('\n')
