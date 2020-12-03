#!/usr/bin/env python3

SHIFT_X = 3
SHIFT_Y = 1

filepath = 'input.txt'
map = open(filepath).read().split('\n')

height = len(map)
width = len(map[0])

x = 0
y = 0

trees_cnt = 0

while y < height:
  if map[y][x] == '#':
    trees_cnt += 1
  x = (x + SHIFT_X) % width
  y += SHIFT_Y

print(trees_cnt)