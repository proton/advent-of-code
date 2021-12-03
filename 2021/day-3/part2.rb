#!/usr/bin/env ruby

def most_common_element(arr)
  h = arr.each_with_object(Hash.new(0)) { |e, h| h[e] += 1 }
  h[?1] >= h[?0] ? ?1 : ?0
end

def least_common_element(arr)
  h = arr.each_with_object(Hash.new(0)) { |e, h| h[e] += 1 }
  h[?1] < h[?0] ? ?1 : ?0
end

def most_common_bit(numbers, position)
  numbers.map { |n| n[position] }.yield_self { |arr| most_common_element(arr) }
end

def least_common_bit(numbers, position)
  numbers.map { |n| n[position] }.yield_self { |arr| least_common_element(arr) }
end

numbers = File.open('input.txt').read.split("\n")
number_length = numbers[0].size

gamma_rate_numbers = numbers
number_length.times do |i|
  bit = most_common_bit(gamma_rate_numbers, i)
  gamma_rate_numbers = gamma_rate_numbers.select { |n| n[i] == bit } if gamma_rate_numbers.size > 1
end

epsilon_rate_numbers = numbers
number_length.times do |i|
  bit = least_common_bit(epsilon_rate_numbers, i)
  epsilon_rate_numbers = epsilon_rate_numbers.select { |n| n[i] == bit } if epsilon_rate_numbers.size > 1
end

gamma_rate = gamma_rate_numbers.first.to_i(2)
epsilon_rate = epsilon_rate_numbers.first.to_i(2)

p gamma_rate * epsilon_rate