# frozen_string_literal: true

require 'set'
require_relative '../ruby_helpers'

GRAPH_RAW = File.readlines('input.txt', chomp: true)
                .map { |l| l.split('->').map(&:strip) }
                .map { |value, key| [key, value.split(' ').map(&:strip)] }

OPS = Set.new(%w[AND OR LSHIFT RSHIFT NOT])

def apply(operation, args, graph)
  parsed = args.map { |a| arg_parse(a, graph) }
  case operation
  when nil
    parsed[0]
  when 'NOT'
    bitsy = '%016d' % parsed[0].to_s(2)
    bitsy.split('').map { |b| b == '0' ? '1' : '0' }.join('').to_i(2)
  when 'AND'
    parsed.reduce(&:&)
  when 'OR'
    parsed.reduce(&:|)
  when 'LSHIFT'
    parsed.reduce(&:<<)
  when 'RSHIFT'
    parsed.reduce(&:>>)
  else
    raise ArgumentError
  end
end

def arg_parse(arg, graph)
  if graph.key?(arg)
    result = evaal(graph[arg], graph)
    # MEMOIZATION IS KEY!
    graph[arg] = [result]
    result
  else
    Integer(arg)
  end
end

def evaal(inst, graph)
  operation, args = inst.partition { |i| OPS.include?(i) }
  apply(operation[0], args, graph)
end

graph_a = GRAPH_RAW.to_h
puts "Part A: #{evaal(graph_a['a'], graph_a)}"

graph_b = GRAPH_RAW.to_h
graph_b['b'] = [16_076]
puts "Part B: #{evaal(graph_b['a'], graph_b)}"
