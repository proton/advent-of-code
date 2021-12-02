#!/usr/bin/env ruby

x, y = 0, 0
aim = 0

data = File.open('input.txt').read.split("\n").each do |line|
  command, change = line.split
  change = change.to_i
  case command
  when "forward"
    x += change
    y += aim * change
  when "down"
    aim += change
  when "up"
    aim -= change
  end
end

puts x*y