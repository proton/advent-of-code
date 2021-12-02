#!/usr/bin/env ruby

numbers = File.open('input.txt').read.split.map(&:to_i)

cnt = 0
numbers.each_cons(3).map(&:sum).each_cons(2) do |(x, y)|
  cnt += 1 if y > x
end

puts cnt