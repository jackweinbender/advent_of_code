#! /usr/bin/env ruby

require 'set'

def parse_line(str)
  from, to, distance = str.split(/\sto\s|\s=\s/)
  return [from.chomp, to.chomp, distance.to_i]
end
class Path
  attr_accessor :distance, :current_city
  def initialize
    @distance = nil
    @current_city = nil
  end

  def go_to(city)
    if @current_city.nil?
      @distance = 0
      @current_city = city
    else
      @distance += @current_city.distance_to(city)
      @current_city = city
    end
    self
  end
end
class City
  attr_reader :name
  def initialize(name)
    @name = name
    @distances = []
  end

  def add_city(name, distance)
    @distances << {name: name, distance: distance}
    @distances.sort_by{|d| d[:distance] }
  end

  def nearest_unvisited_city(visited)
    @distances.find(proc {false}){|d| !visited.include?(d[:name]) }
  end

  def distance_to(city)
    to = @distances.find{ |dist| city.name == dist[:name] }
    to[:distance]
  end
end

input = $stdin.readlines(chomp: true)
  .map { |l| parse_line(l) }
  .reduce(Hash.new([])){|acc, nxt|
    from, to, distance = nxt

    acc[from] = City.new(from) unless acc.has_key?(from)
    acc[to] = City.new(to) unless acc.has_key?(to)

    acc[from].add_city(to, distance)
    acc[to].add_city(from, distance)
    acc
  }

# Nearest Neigbor Algo. Not Exact. Doesn't work for full input. :(
part_one_nn = input.values.map do |city|
  distance = 0
  visited = Set.new
  visited.add(city.name)

  while city.nearest_unvisited_city(visited) do
    nearest = city.nearest_unvisited_city(visited)
    distance += nearest[:distance]
    visited.add(nearest[:name])
    city = input[nearest[:name]]
  end

  distance
end


bruteforce_paths = input.values.permutation.to_a.map{|path|
  path.reduce(Path.new){ |acc, nxt|
    acc.go_to(nxt)
  }
}.map(&:distance)

puts "Part 1: #{bruteforce_paths.min}"
puts "Part 2: #{bruteforce_paths.max}"
