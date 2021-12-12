#!/usr/bin/env ruby

def lowest?(data, y, x)
  n = data[y][x]
  return false if y > 0 && data[y - 1][x] <= n
  return false if y < data.size - 1 && data[y + 1][x] <= n
  return false if x > 0 && data[y][x - 1] <= n
  return false if x < data[0].size - 1 && data[y][x + 1] <= n
  true
end

data = File.open('input.txt').readlines.map { |l| l.strip.chars.map(&:to_i) }

arr = []

data.size.times    do |y|
data[0].size.times do |x|
  arr << data[y][x] if lowest?(data, y, x)
end
end

# p data
p arr.map { |n| n + 1 }.sum

# h = { 
#   # 6 => 0,
#   2 => 1,
#   # 5 => 2,
#   # 5 => 3,
#   4 => 4,
#   # 5 => 5,
#   # 6 => 6,
#   3 => 7,
#   7 => 8
#   # 6 => 9,
# }

# data = File.open('input.txt').readlines.map { |l| l.split('|').last }.join(' ').split
# r = data.map { |s| h[s.size] }.compact.size

# puts r