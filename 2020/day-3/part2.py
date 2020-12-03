#!/usr/bin/env python3

shifts = [
  { "x": 1, "y": 1 },
  { "x": 3, "y": 1 },
  { "x": 5, "y": 1 },
  { "x": 7, "y": 1 },
  { "x": 1, "y": 2 }
]

filepath = 'input.txt'
map = open(filepath).read().split('\n')

height = len(map)
width = len(map[0])

trees_multiplication = 1

for shift in shifts:
  trees_cnt = 0
  x = 0
  y = 0

  while y < height:
    if map[y][x] == '#':
      trees_cnt += 1
    x = (x + shift["x"]) % width
    y += shift["y"]

  trees_multiplication *= trees_cnt
  print(trees_cnt)

print(trees_multiplication)