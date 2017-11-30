
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
end

input = File.readlines('input.txt')
    .map { |l| Node.new(l) }
    .sort {|x,y| x.avail <=> y.avail }

# available.each{|x| puts x.pr }


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