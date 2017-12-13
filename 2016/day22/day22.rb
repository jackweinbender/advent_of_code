
require 'set'

class Node
    attr_accessor :pos, :size, :used, :avail
    def initialize(str)
        f, *d = str.split()
        @pos = get_pos(f)
        @size = detee(d[0])
        @used = detee(d[1])
        @avail = @size - @used
    end
    def detee(str)
        return str[0..str.length-2].to_i
    end
    def get_pos(str)
        str = str.split("-")
        str.shift
        return str.map { |x| x[1..x.length - 1].to_i }
    end
    def pr 
        return "Pos: #{@pos}, Size: #{@size}, Used: #{@used}, Avail: #{@avail}"
    end
    def to_symbol
        if @used == 0
            return "_"
        elsif @used <= 94
            return "."
        else
            return "#"
        end
    end
end

input = File.readlines('input.txt')
    .map { |l| Node.new(l) }
    .sort {|x,y| 
        if x.pos[1] == y.pos[1]
            x.pos[0] <=> y.pos[0]
        else
            x.pos[1] <=> y.pos[1]
        end
    }

## Answer #1 ##
pairs = Set.new []
for u in input
    for a in input
        if a == u
            next
        elsif u.used == 0
            next
        elsif a.avail >= u.used
            pairs.add([a,u].to_set)
        else
            next
        end
    end
end
puts "Answer #1: #{pairs.length()}"

rows = input.reduce(Array.new()) { |acc,node| 
    if node.pos[0] == 0
        acc.push([])
    end
    acc[-1].push(node.to_symbol)
    acc
}

# Gotta figure this out by hand after printing to screen
# 53 to reach the top right
# 5x 29 to each the top left == 145
# == 198
rows.each { |r|
    r.each {|c| print c}
    print "\n"
}