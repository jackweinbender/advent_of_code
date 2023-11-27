#! /usr/bin/env ruby
require_relative '../ruby_helpers'
require 'set'

input = $stdin.readlines(chomp: true)

def parse_line(string)
  arr = string.gsub(".", '').split(" ")
  subject   = arr[0]
  operator  = arr[2] == 'gain' ? 1 : -1
  value     = arr[3].to_i * operator
  object    = arr[-1]

  [subject, value, object]
end

tests 'parse input' do
  assert(parse_line('Alice would gain 54 happiness units by sitting next to Bob.') == ["Alice", 54, "Bob"])
  assert(parse_line('Alice would lose 79 happiness units by sitting next to Carol.') == ["Alice", -79, "Carol"])
end

class Person
  attr_reader :id, :relations
  def initialize(name)
    @id = name
    @relations = {}
  end
  def add(person_id, value)
    @relations[person_id] = value
  end
  def with(person_id)
    @relations[person_id]
  end
end

people = {}

# build the map of people and their preferences
input.each do |line|
  s,v,o = parse_line(line)
  if people.include? s
    people[s].add(o, v)
  else
    person = Person.new(s)
    person.add(o,v)
    people[s] = person
  end
end

def table_pairs(arr)
  iter = arr.cycle
  arr.map{ |el| [iter.next, iter.peek].sort }.sort
end

# pairwise combinations of people, with the net happiness score
pairs = people.keys.combination(2).map{ |a,b| [[a,b], people[a].with(b) + people[b].with(a) ]}.to_h
combos = people.keys.permutation.map{|arr| table_pairs(arr)}.uniq

part_one = combos.map{ |ordering| ordering.map{ |pair| pairs[pair] }.sum }.max

puts "Part 1: #{part_one}"

# Part 2

people.values.each do |person|
  person.add('me', 0)
end

me = Person.new('me')
people.keys.each do |p|
  me.add(p, 0)
end
people[me.id] = me

# pairwise combinations of people, with the net happiness score
pairs = people.keys.combination(2).map{ |a,b| [[a,b], people[a].with(b) + people[b].with(a) ]}.to_h
combos = people.keys.permutation.map{|arr| table_pairs(arr)}.uniq

part_two = combos.map{ |ordering| ordering.map{ |pair| pairs[pair] }.sum }.max

puts "Part 2: #{part_two}"
