# frozen_string_literal: true

require_relative '../ruby_helpers'

def filter_from(string)
  opcode, *tail = string.split(' ')
  opcode, *tail = tail unless opcode == 'toggle'

  start, stop = [tail[0], tail[2]].map { |p| p.split(',') }

  func = case opcode
         when 'toggle'
           ->(value) { value.zero? ? 1 : 0 }
         when 'on'
           ->(_) { 1 }
         when 'off'
           ->(_) { 0 }
         else
           raise ArgumentError
         end

  Filter.new(start, stop, &func)
end

# Filter
class Filter
  attr_reader :range

  def initialize(start, stop, &block)
    @range = set_range(start, stop)
    @func = block
  end

  def apply(bool)
    @func.call(bool)
  end

  def set_range(start, stop)
    @start_col, @start_row = start
    @stop_col, @stop_row = stop

    Enumerator.new do |yielder|
      (@start_row..@stop_row).each do |row|
        (@start_col..@stop_col).each do |col|
          yielder << [row.to_i, col.to_i]
        end
      end
    end
  end
end

# Grid Class
class Grid
  def initialize(width = 1000, height = 1000)
    @grid = Array.new(height) { Array.new(width, 0) }
  end

  def apply(filter)
    filter.range.each do |row, col|
      @grid[row][col] = filter.apply(@grid[row][col])
    end
  end

  def sum_on
    @grid.flatten.sum
  end
end

input = File.readlines('input.txt', chomp: true)

tests 'part 1' do
  g = Grid.new(10, 10)
  on = filter_from('turn on 4,4 through 5,5')
  off = filter_from('turn off 0,0 through 9,9')
  toggle = filter_from('toggle 0,0 through 9,9')

  g.apply(on)
  assert g.sum_on == 4

  g.apply(toggle)
  assert g.sum_on == 96

  g.apply(off)
  assert g.sum_on == 0

  g.apply(toggle)
  assert g.sum_on == 100
end

def filter_from_rev(string)
  opcode, *tail = string.split(' ')
  opcode, *tail = tail unless opcode == 'toggle'

  start, stop = [tail[0], tail[2]].map { |p| p.split(',') }

  func = case opcode
         when 'toggle'
           ->(value) { value + 2 }
         when 'on'
           ->(value) { value + 1 }
         when 'off'
           ->(value) { value <= 0 ? 0 : value - 1 }
         else
           raise ArgumentError
         end

  Filter.new(start, stop, &func)
end

part1 = Grid.new
input.each { |op| part1.apply(filter_from(op)) }

part2 = Grid.new
input.each { |op| part2.apply(filter_from_rev(op)) }

results do
  puts "Part 1: #{part1.sum_on}".blue
  puts "Part 2: #{part2.sum_on}".blue
end
