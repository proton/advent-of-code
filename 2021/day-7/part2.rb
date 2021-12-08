#!/usr/bin/env ruby

@h = {}
def cost(n)
  n = n.abs
  @h[n] ||= n.downto(0).reduce(:+)
end

state = File.open('input.txt').readline.split(',').map(&:to_i)
nums = state.uniq
r = (nums.min..nums.max).map do |num|
  state.map { |s| cost(s-num) }.sum
end.min

p r