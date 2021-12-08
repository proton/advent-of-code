#!/usr/bin/env ruby

state = File.open('input.txt').readline.split(',').map(&:to_i).sort
nums = state.uniq
r = nums.map do |num|
  state.map { |s| (s-num).abs }.sum
end.min

puts r