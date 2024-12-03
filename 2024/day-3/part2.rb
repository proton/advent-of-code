#!/usr/bin/env ruby
require 'open-uri'

raw = open('input.txt').read
instructions = raw.scan(/mul\(\d+,\d+\)|don?'?t?\(\)/)

enabled = true

sum = 0

instructions.each do |instruction|
  if instruction == "do()"
    enabled = true
  elsif instruction == "don't()"
    enabled = false
  elsif enabled
    a, b = instruction.scan(/\d+/).map(&:to_i)
    sum += a * b
  end
end

p sum