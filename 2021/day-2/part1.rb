#!/usr/bin/env ruby

x, y = 0, 0
data = File.open('input.txt').read.split("\n").each do |line|
  command, change = line.split
  change = change.to_i
  case command
  when "forward" then x += change
  when "down"    then y += change
  when "up"      then y -= change
  end
end

puts x*y