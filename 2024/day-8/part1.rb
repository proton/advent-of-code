#!/usr/bin/env ruby
require 'open-uri'

rows = open('input.txt').readlines.map(&:chomp)

positions = {}

busy = Set.new

rows.each_with_index do |row, y|
  row.each_char.with_index do |char, x|
    next if char == '.' || char == '#'

    positions[char] ||= []
    positions[char] << [y, x]
  end
end

height = rows.size
width  = rows[0].size

cnt = 0

positions.each do |char, coords|
  next if coords.size < 2

  coords.each_with_index do |coord1, i|
    coords[(i + 1)..-1].each do |coord2|
      y1, x1 = coord1
      y2, x2 = coord2

      y0 = y1 - (y2 - y1)
      x0 = x1 - (x2 - x1)
      y3 = y2 + (y2 - y1)
      x3 = x2 + (x2 - x1)

      [[y0, x0], [y3, x3]].each do |y, x|
        next if y < 0 || y >= height || x < 0 || x >= width
        next if busy.include?([y, x])

        cnt += 1
        busy << [y, x]
      end
    end
  end
end

puts cnt