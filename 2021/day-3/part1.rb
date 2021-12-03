#!/usr/bin/env ruby

def most_common_element(arr)
  arr.sort[arr.size/2]
end

def least_common_element(arr)
  h = arr.each_with_object(Hash.new(0)) { |e, h| h[e] += 1 }
  h.min_by { |k,v| v}.first
end

numbers = File.open('input.txt').read.split("\n")

gamma_rate = numbers[0].size.times.map do |i|
  numbers.map { |n| n[i] }.yield_self { |arr| most_common_element(arr) }
end.join.to_i(2)

epsilon_rate = numbers[0].size.times.map do |i|
  numbers.map { |n| n[i] }.yield_self { |arr| least_common_element(arr) }
end.join.to_i(2)

p gamma_rate * epsilon_rate