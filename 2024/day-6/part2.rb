#!/usr/bin/env ruby
require 'open-uri'

board = open('input.txt').readlines.map(&:chomp)

height = board.size
width  = board[0].size

change_direction = lambda do |dy, dx|
  if dy === 0
    dy = dx
    dx = 0
  else
    dx = -dy
    dy = 0
  end

  [dy, dx]
end

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

original_board = board.map(&:dup)

possible_locations = []

sx, sy = find_start.call(board)

x, y = sx, sy
dy, dx = -1, 0

while x >= 0 && y >=0 && x < width && y < height
  cell = board[y][x]
  case cell
  when '.'
    board[y][x] = 'X'
    
    possible_locations << [y, x]

    x += dx
    y += dy
  when 'X'
    x += dx
    y += dy
  when '#'
    x -= dx
    y -= dy

    dy, dx = change_direction.call(dy, dx)
  end
end

has_loop = lambda do |board, by, bx|
  visited = Set.new

  y, x = sy, sx
  dy, dx = -1, 0

  while x >= 0 && y >=0 && x < width && y < height
    if (x === bx && y === by) || board[y][x] === '#'
      x -= dx
      y -= dy
  
      dy, dx = change_direction.call(dy, dx)
    else
      key = [y, x, dy, dx]

      return true if visited.include?(key)

      visited << key
      x += dx
      y += dy
    end
  end

  return false
end

cnt = 0

puts board

board = original_board

possible_locations.each do |by, bx|
  cnt += 1 if has_loop.call(board, by, bx)
end

puts cnt