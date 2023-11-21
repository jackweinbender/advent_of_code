#! /usr/bin/env ruby
require 'set'
require_relative '../ruby_helpers'

input = $stdin.readlines(chomp: true)

part_two = input

# Passwords must include one increasing straight of at least three letters,
  # like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
# Passwords may not contain the letters i, o, or l,
  # as these letters can be mistaken for other characters and are therefore confusing.
# Passwords must contain at least two different,
  # non-overlapping pairs of letters, like aa, bb, or zz.

ILLEGAL_CHARS = Set.new(['i', 'o', 'l'])
def cc(string)
  string.split('').any? { |c| ILLEGAL_CHARS.include?(c) }
end

def nop(string)
  count = 0
  i = 1
  while i < string.length && count < 2 do
    if string[i-1] == string[i]
      count += 1
      i += 1
    end

    i+=1
  end
  return count == 2
end


def consec(arr)
  starting_num = arr.first
  arr.to_enum.with_index.all? { |n, idx| n == starting_num + idx}
end

def has_consec(string, window=3)
  string.split('')
    .map{|c| c.codepoints.first }
    .each_cons(window).any? { |slice| consec(slice) }
end

def has_nonoverlaping_pairs(string)
  nop(string)
end

def has_only_valid_chars(string)
  !cc(string)
end

def check(string)
  return false unless has_only_valid_chars(string)
  return false unless has_nonoverlaping_pairs(string)
  return false unless has_consec(string)

  true
end

def next_pwd(string)
  pwd = string.next
  while check(pwd) == false do
    pwd = pwd.next
  end
  pwd
end

tests 'non-overlapping pairs' do
  assert(nop('abcd') == false)
  assert(nop('abcc') == false)
  assert(nop('aabc') == false)
  assert(nop('aabb') == true)
  assert(nop('aaaa') == true)
end

tests 'confusing-chars' do
  assert(cc('abcd') == false)
  assert(cc('abci') == true)
  assert(cc('abco') == true)
  assert(cc('abcl') == true)
end

tests 'consec' do
  assert(consec([1,2,3]) == true)
  assert(consec([1]) == true)
  assert(consec([1,2,3,4,5]) == true)
  assert(consec([1,2,3,5]) == false)
end

tests 'has-consecutive' do
  assert(has_consec('abcd') == true)
  assert(has_consec('abdc') == false)
  assert(has_consec('abde') == false)
  assert(has_consec('bbcd') == true)
end


tests 'hijklmmn' do
  assert(has_consec('hijklmmn') == true)
  assert(cc('hijklmmn') == true)
  assert(nop('hijklmmn') == false)
end

tests 'abbceffg' do
  assert(has_consec('abbceffg') == false)
  assert(cc('abbceffg') == false)
  assert(nop('abbceffg') == true)
end

tests 'next after abcdefgh' do
  assert(next_pwd('abcdefgh') == 'abcdffaa')
end

part_one = next_pwd(input.first)
part_two = next_pwd(part_one)
puts "Part 1: #{part_one}"
puts "Part 2: #{part_two}"
