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

    busy << [y, x]
  end
end

height = rows.size
width  = rows[0].size

positions.each do |char, coords|
  next if coords.size < 2

  coords.each_with_index do |coord1, i|
    coords[(i + 1)..-1].each do |coord2|
      y1, x1 = coord1
      y2, x2 = coord2

      ydiff = y2 - y1
      xdiff = x2 - x1

      y, x = y1, x1
      while true
        y -= ydiff
        x -= xdiff

        break if y < 0 || y >= height || x < 0 || x >= width

        busy << [y, x]
      end

      y, x = y2, x2
      while true
        y += ydiff
        x += xdiff

        break if y < 0 || y >= height || x < 0 || x >= width

        busy << [y, x]
      end
    end
  end
end

puts busy.size