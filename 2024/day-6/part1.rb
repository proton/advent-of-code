#!/usr/bin/env ruby
require 'open-uri'

board = open('input.txt').readlines.map(&:chomp)

height = board.size
width  = board[0].size

find_start = lambda do |board|
  board.each_with_index do |row, y|
    row.each_char.with_index do |cell, x|
      if cell == '^'
        board[y][x] = '.'
        return [x, y]
      end
    end
  end
end


cnt = 0

x, y = find_start.call(board)
dy, dx = -1, 0

while x >= 0 && y >=0 && x < width && y < height
  cell = board[y][x]
  case cell
  when '.'
    board[y][x] = 'X'
    cnt += 1
    x += dx
    y += dy
  when 'X'
    x += dx
    y += dy
  when '#'
    x -= dx
    y -= dy

    if dy === 0
      dy = dx
      dx = 0
    else
      dx = -dy
      dy = 0
    end
  end
end

puts cnt