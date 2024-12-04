#!/usr/bin/env ruby
require 'open-uri'

data = open('input.txt').readlines.map(&:chomp)

height = data.size
width  = data[0].size

is_mas = lambda do |y1, x1, y2, x2|
  return false if y1 < 0 || y1 >= height || x1 < 0 || x1 >= width
  return false if y2 < 0 || y2 >= height || x2 < 0 || x2 >= width

  a = data[y1][x1]
  b = data[y2][x2]
  
  (a === ?M && b === ?S) || (a === ?S && b === ?M)
end

xmas_present = lambda do |row, col|
  is_mas.call(row - 1, col - 1, row + 1, col + 1) && is_mas.call(row + 1, col - 1, row - 1, col + 1)
end

xmas_cnt = 0

data.each_with_index do |line, row|
  line.each_char.with_index do |char, col|
    next unless char == 'A'

    xmas_cnt += 1 if xmas_present.call(row, col)
  end
end

puts xmas_cnt