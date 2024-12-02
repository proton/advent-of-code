#!/usr/bin/env ruby
require 'open-uri'

reports = open('input.txt').readlines

def test(levels)
  levels_sign = nil

  levels.each_cons(2) do |a, b|
    diff = b - a
    sign = diff <=> 0

    levels_sign = sign if levels_sign.nil?
    
    return false if sign == 0
    return false unless sign == levels_sign
    return false if diff.abs > 3
  end
  
  true
end

def safe_report?(report)
  levels = report.split.map(&:to_i)
  return true if levels.size < 3

  if test(levels)
    return true
  else
    levels.each_index do |i|
      levels_a = levels.dup
      levels_a.delete_at(i)
      return true if test(levels_a)
    end
  end

  return false
end

safe_reports_cnt = reports.count do |report|
  safe_report?(report)
end

puts safe_reports_cnt