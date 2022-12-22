#! /usr/bin/env ruby
input = STDIN.read.split("\n")

class RPS
  def self.from(letter)
    case letter
    when 'A', 'X'
      Rock.new
    when 'B', 'Y'
      Paper.new
    when 'C', 'Z'
      Scissors.new
    else
      raise ArgumentError
    end
  end

  def against(player)
    if type == player.beats
      player.points + 6
    elsif type == player.type
      player.points + 3
    else
      player.points
    end
  end

  def points
    raise NotImplementedError
  end

  def type
    raise NotImplementedError
  end

  def beats
    raise NotImplementedError
  end
end

class Rock < RPS
  def points
    1
  end

  def type
    :rock
  end

  def beats
    :scissors
  end
end

class Paper < RPS
  def points
    2
  end

  def type
    :paper
  end

  def beats
    :rock
  end
end

class Scissors < RPS
  def points
    3
  end

  def type
    :scissors
  end

  def beats
    :paper
  end
end

parsed = input.map do |line|
  line.split(' ').map { |letter| RPS.from(letter) }
end

puts "Part 1: #{parsed.map { |player, pone| player.against(pone) }.sum}"
