#!/usr/bin/env ruby
require 'open-uri'

raw = open('input.txt').read
instructions = raw.scan(/mul\(\d+,\d+\)/)

sum = 0

instructions.each do |instruction|
  a, b = instruction.scan(/\d+/).map(&:to_i)
  sum += a * b
end

p sum