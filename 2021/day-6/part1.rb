#!/usr/bin/env ruby

state = File.open('input.txt').readline.split(',').map(&:to_i)

80.times do
  p state
  new_fishes = 0
  state.each_index do |i|
    state[i] -= 1
    if state[i] == -1
      state[i] = 6
      new_fishes += 1
    end
  end
  state += new_fishes.times.map { 8 }
end

p state.size