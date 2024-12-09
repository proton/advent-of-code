#!/usr/bin/env ruby
require 'open-uri'

data = open('input.txt').read

res = []
increment = 0
data.each_char.with_index do |size, i|
  id = nil
  if i % 2 === 0
    id = increment
    increment += 1
  end

  size.to_i.times { res << id }
end

i = 0
j = res.size - 1

while i < j
  if res[i]
    i += 1
  elsif res[j].nil?
    j -= 1
  else
    res[i], res[j] = res[j], res[i]
    i += 1
    j -= 1
  end
end

res.compact!

checksum = 0
res.each.with_index do |val, i|
  checksum += i * val
end

puts checksum