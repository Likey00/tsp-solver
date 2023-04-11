with open('test48.txt') as f:
    matrix = [[int(x) for x in line.split()] for line in f]

with open('final_path.txt') as f:
    cost = 0
    prev = -1
    for line in f:
        if prev == -1:
            prev = int(line.strip())
            continue
        cost += matrix[int(line.strip())-1][prev-1]
        prev = int(line.strip())
    print(cost)