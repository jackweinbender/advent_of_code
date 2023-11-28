#! /usr/bin/env ruby

class Deer
include Comparable
  attr_reader :wins
  def initialize(name, speed, endurance, rest)
    @name = name
    @speed = speed
    @endurance = endurance
    @rest = rest
    @wins = 0
  end

  def distance_after(seconds)
    interval = @endurance + @rest
    # Total full fly/rest cycles
    interval_count = seconds / interval
    # partial moving time on last incomplete cycle
    # which is the lesser of the @endurance or mod
    interval_mod = [@endurance, seconds % interval].min
    moving_time = (@endurance * interval_count) + interval_mod

    moving_time * @speed
  end
  def tick
    @wins += 1
  end
end

input = $stdin.readlines(chomp: true)
  .map do |l|
    line = l.split(" ")
    Deer.new(line[0], line[3].to_i, line[6].to_i, line[13].to_i)
  end

part_one = input.map{|deer| deer.distance_after(2503)}.max

2503.times do |iteration|
  second = iteration + 1
  winner_distance = input.map { |d| d.distance_after(second) }.max
  winners = input
    .select{|d| d.distance_after(second) == winner_distance }
    .each {|d| d.tick }
end

part_two = input.map { |d| d.wins }.max

puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
