#!/usr/bin/env ruby

require 'ostruct'

def pretty_print(map)
  puts ' '
  puts map.map {|l| l.join(' ')}
end

lines = File.open('input.txt').read.split("\n").map do |line|
  coords = line.split(' -> ')
  s = OpenStruct.new
  s.start = coords.first.split(',').map(&:to_i)
  s.end   = coords.last.split(',').map(&:to_i)
  s
end

min_x = min_y = 0
max_x = lines.map { |l| [l.start[0], l.end[0]] }.flatten.max
max_y = lines.map { |l| [l.start[1], l.end[1]] }.flatten.max

map = Array.new(max_y + 1)
map.each_index { |i| map[i] = Array.new(max_x + 1, 0) }

lines.each do |line|
  x_mod = line.end[0] <=> line.start[0]
  y_mod = line.end[1] <=> line.start[1]
  coords = line.start
  while coords != line.end
    map[coords[1]][coords[0]] += 1
    coords[0] += x_mod
    coords[1] += y_mod
  end
  map[line.end[1]][line.end[0]] += 1
end

pretty_print map

p map.flatten.count { |cnt| cnt >= 2 }