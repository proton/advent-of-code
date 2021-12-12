#!/usr/bin/env ruby

h = { 
  # 6 => 0,
  2 => 1,
  # 5 => 2,
  # 5 => 3,
  4 => 4,
  # 5 => 5,
  # 6 => 6,
  3 => 7,
  7 => 8
  # 6 => 9,
}

data = File.open('input.txt').readlines.map { |l| l.split('|').last }.join(' ').split
r = data.map { |s| h[s.size] }.compact.size

puts r