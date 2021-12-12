#!/usr/bin/env ruby

def include?(big, small)
  small.split('') - big.split('') == []
end

h = { 
  # 6 => 0,
  2 => 1,
  # 5 => 2,
  # 5 => 3,
  4 => 4,
  # 5 => 5,
  # 6 => 6,
  3 => 7,
  7 => 8
  # 6 => 9,
}

lines = File.open('input.txt').readlines #.map { |l| l.split('|').last } #.join(' ').split

a = lines.map do |line|
  numbers = line.gsub('|', '').split.map { |n| n.chars.sort.join }.uniq
  # p numbers
  
  m = {}

  numbers.each do |n|
    m[h[n.size]] = n if h[n.size]
  end

  # p numbers.uniq

  # p m
  m[0] = numbers.detect { |n| n.size == 6 && !include?(n, m[4]) && include?(n, m[1]) }
  m[6] = numbers.detect { |n| n.size == 6 && !include?(n, m[4]) && !include?(n, m[1]) }
  m[9] = numbers.detect { |n| n.size == 6 && include?(n, m[4]) }

  m[3] = numbers.detect { |n| n.size == 5 && include?(n, m[1]) }
  m[5] = numbers.detect { |n| n.size == 5 && !include?(n, m[1]) && include?(m[6], n) }
  m[2] = numbers.detect { |n| n.size == 5 && !include?(n, m[1]) && !include?(m[6], n) }

  mr = m.invert

  line.split('|').last.split.map { |n| mr[n.chars.sort.join] }.join.to_i
end

p a.sum

# r = data.map { |s| h[s.size] }.compact.size

# puts r