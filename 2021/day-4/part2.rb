#!/usr/bin/env ruby

class Board
  attr_reader :rows
  attr_reader :columns
  attr_reader :win

  def initialize(lines)
    @rows = lines.map { |l| l.split(' ').map(&:to_i) }
    @columns = lines.each_index.map { |x| rows.map {|row| row[x]} }
    @win = false
  end

  def bingo?(numbers)
    @win = true if rows.any? { |row| (row - numbers).empty? }
    @win = true if columns.any? { |column| (column - numbers).empty? }
    @win
  end

  def unmarked_numbers(numbers)
    all_numbers - numbers
  end

  private

  def all_numbers
    rows.reduce(:+)
  end
end


lines = File.open('input.txt').read.split("\n")

numbers = lines[0].split(',').map(&:to_i)

boards = (lines.size/6).times.map do |n|
  x = 2 + n * 6
  y = 6 + n * 6
  Board.new(lines[x..y])
end

(1..numbers.size).each do |upto|
  ns = numbers[0...upto]
  unwin_boards = boards.reject(&:win)
  unwin_boards.each do |board|
    if board.bingo?(ns)
      p ns.last * board.unmarked_numbers(ns).sum
    end
  end
end