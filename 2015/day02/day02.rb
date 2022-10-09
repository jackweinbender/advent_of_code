input = File.read('input.txt')
            .split("\n")
            .map(&:strip)
            .map { |l| l.split('x').map(&:to_i) }

def area(length, width)
  length * width
end

def surface_area(len, width, height)
  [area(len, width), area(width, height), area(len, height)].map { |i| i * 2 }
end

def perimeter(line)
  line.min(2).sum * 2
end

def volume(length, width, height)
  length * width * height
end

def total_wrapping_paper(box)
  box.sum + (box.min / 2)
end

def total_ribbon(line)
  perimeter(line) + volume(*line)
end

paper = input.map { |line| surface_area(*line) }
             .map { |box| total_wrapping_paper(box) }.sum

ribbon = input.map { |line| total_ribbon(line) }.sum

puts "Necessary Paper: #{paper}"
puts "Necessary Ribbon: #{ribbon}"
