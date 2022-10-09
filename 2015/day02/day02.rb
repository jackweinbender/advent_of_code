input = File.read('input.txt')
            .split("\n")
            .map(&:strip)
            .map { |l| l.split('x').map(&:to_i) }

def area(length, width)
  length * width
end

def volume(length, width, height)
  length * width * height
end

def faces(len, width, height)
  [area(len, width), area(width, height), area(len, height)]
end

def shortest_perimeter(line)
  line.min(2).sum * 2
end

def total_wrapping_paper(line)
  (faces(*line).sum * 2) + faces(*line).min
end

def total_ribbon(line)
  shortest_perimeter(line) + volume(*line)
end

paper  = input.map { |line| total_wrapping_paper(line) }.sum
ribbon = input.map { |line| total_ribbon(line) }.sum

puts "Necessary Paper: #{paper}"
puts "Necessary Ribbon: #{ribbon}"
