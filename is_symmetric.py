with open('test.txt') as f:
    matrix = []
    for line in f:
        row = [float(x) for x in line.split()]
        matrix.append(row)


for i in range(len(matrix)):
    for j in range(i+1, len(matrix)):
        if matrix[i][j] != matrix[j][i]:
            print('NOT SYMMETRIC')