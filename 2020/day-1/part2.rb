#!/usr/bin/env ruby
require 'open-uri'

numbers = open('input.txt').read.split.map(&:to_i)

numbers.each do |x|
numbers.each do |y|
numbers.each do |z|
  next if x == y || y == z || z == x
  puts x * y * z if x + y + z == 2020
end
end
end