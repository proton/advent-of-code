#!/usr/bin/env ruby
require 'open-uri'

data = open('input.txt').readlines.map(&:chomp)

height = data.size
width  = data[0].size

directions = [
  [1, 1], [1, 0], [1, -1], [0, 1], [0, -1], [-1, 1], [-1, 0], [-1, -1]
]

xmas = 'XMAS'

xmas_present = lambda do |row, col, dy, dx|
  y_end = row + dy * (xmas.size - 1)
  x_end = col + dx * (xmas.size - 1)

  return false if y_end < 0 || y_end >= height || x_end < 0 || x_end >= width

  xmas.each_char.with_index do |char, i|
    y = row + dy * i
    x = col + dx * i
    return false if data[y][x] != char
  end

  true
end

xmas_cnt = 0

data.each_with_index do |line, row|
  line.each_char.with_index do |char, col|
    next unless char == 'X'

    directions.each do |dy, dx|
      xmas_cnt += 1 if xmas_present.call(row, col, dy, dx)
    end
  end
end

puts xmas_cnt