#!/usr/bin/env ruby
require 'open-uri'

numbers = open('input.txt').read.split.map(&:to_i)

numbers.each do |x|
numbers.each do |y|
  next if x == y
  puts x * y if x + y == 2020
end
end