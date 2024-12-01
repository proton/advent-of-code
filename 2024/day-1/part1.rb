#!/usr/bin/env ruby
require 'open-uri'

rows = open('input.txt').readlines

arr1 = Array.new(rows.size)
arr2 = Array.new(rows.size)

rows.each_with_index do |row, i|
  a, b = row.split.map(&:to_i)
  arr1[i] = a
  arr2[i] = b
end

arr1.sort!
arr2.sort!

total = 0

arr1.each_with_index do |a, i|
  total += (arr2[i] - a).abs
end

puts total