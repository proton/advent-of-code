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

  res << [id, size.to_i]
end

j = res.size - 1
while j >= 0
  part = res[j]
  if part[0].nil?
    j -= 1
    next
  end

  size = part[1]

  for i in 0...j do
    space = res[i]
    if space[0].nil? && space[1] >= size
      res[j] = [nil, size]
      if space[1] == size
        res[i] = part
      else
        res.insert(i, part)
        space[1] -= size
      end
      break
    end
  end
  
  j -= 1
end

res2 = []
res.each do |part|
  next if part.nil?
  
  val, cnt = part
  cnt.times { res2 << val }
end
res = res2

checksum = 0
res.each.with_index do |val, i|
  next if val.nil?
  checksum += i * val
end

puts checksum