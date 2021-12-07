#!/usr/bin/env ruby

state = File.open('input.txt').readline.split(',').map(&:to_i)
years = 256

h = Hash.new(0)
state.each { |c| h[c] += 1 }

years.times do |x|
  h2 = Hash.new(0)
  h.each do |k, cnt|
    if k == 0
      h2[6] += cnt
      h2[8] += cnt
    else
      h2[k - 1] += cnt
    end
  end
  h = h2
end

p h.values.sum