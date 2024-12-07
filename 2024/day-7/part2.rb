#!/usr/bin/env ruby
require 'open-uri'

rows = open('input.txt').readlines.map(&:chomp)

cnt = 0

test = lambda do |result, numbers|
  queue = []

  numbers.each do |n|
    if queue.empty?
      queue << n
    else
      next_queue = []
      queue.each do |val|
        a = val + n
        b = val * n
        c = val * 10 ** n.to_s.size + n
        [a, b, c].each do |x|
          next_queue << x if x <= result
        end
      end
      queue = next_queue
    end
  end

  return queue.include?(result)
end

rows.each do |row|
  result, *numbers = row.split(/:? /).map(&:to_i)
  cnt += result if test.call(result, numbers)
end

puts cnt