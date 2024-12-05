#!/usr/bin/env ruby
require 'open-uri'

data = open('input.txt').readlines.map(&:chomp)

rules = data.select { |line| line.include?('|') }
rows  = data.select { |line| line.include?(',') }.map { |line| line.split(',').map(&:to_i) }

befores = {}

rules.each do |rule|
  a, b = rule.split('|').map(&:to_i)

  befores[a] = Set.new if befores[a].nil?
  befores[a] << b
end

result = 0

is_good_row = lambda do |row|
  prevs = Set.new

  row.each do |num|
    if befores[num] && prevs.intersect?(befores[num])
      return false
    end

    prevs << num
  end

  return true
end

rows.each do |row|
  result += row[row.size / 2] if is_good_row.call(row)
end

puts result