#!/usr/bin/env ruby
require 'open-uri'

reports = open('input.txt').readlines

def safe_report?(report)
  levels = report.split.map(&:to_i)

  levels_sign = nil
  levels.each_cons(2) do |a, b|
    diff = b - a
    sign = diff <=> 0

    levels_sign = sign if levels_sign.nil?
    
    return false unless sign == levels_sign
    return false if diff.abs > 3
  end
  true
end

safe_reports_cnt = reports.count do |report|
  safe_report?(report)
end

puts safe_reports_cnt