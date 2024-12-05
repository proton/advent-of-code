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

bad_rows = rows.reject { |row| is_good_row.call(row) }

sort = lambda do |row|
  candidates = Set.new(row)

  row = []

  while candidates.size > 0
    num = candidates.find { |num| !candidates.intersect?(befores[num]) }

    candidates.delete(num)

    row << num
  end

  row
end

bad_rows.each do |row|
  row = sort.call(row)

  result += row[row.size / 2]
end

puts result