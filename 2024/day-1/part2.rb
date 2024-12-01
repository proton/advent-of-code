#!/usr/bin/env ruby
require 'open-uri'

rows = open('input.txt').readlines

nums = Array.new(rows.size)
cnts = Hash.new(0)

rows.each_with_index do |row, i|
  a, b = row.split.map(&:to_i)
  nums[i] = a
  cnts[b] += 1
end

total = 0

nums.each do |num|
  total += num * cnts[num]
end

puts total