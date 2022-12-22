class Shape
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

  def self.for_outcome(them, outcome)
    case outcome
    when 'X' # Loss
      them.beats.new
    when 'Y' # Tie
      them.ties.new
    when 'Z' # Win
      them.loses_to.new
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
    self.class
  end

  def beats
    raise NotImplementedError
  end

  def loses_to
    raise NotImplementedError
  end

  def ties
    type
  end
end

class Rock < Shape
  def points
    1
  end

  def beats
    Scissors
  end

  def loses_to
    Paper
  end
end

class Paper < Shape
  def points
    2
  end

  def beats
    Rock
  end

  def loses_to
    Scissors
  end
end

class Scissors < Shape
  def points
    3
  end

  def beats
    Paper
  end

  def loses_to
    Rock
  end
end