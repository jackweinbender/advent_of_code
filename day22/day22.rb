
class Node
    def initialize(str)
        f, *d = str.split()
        @pos = get_pos(f)
        @size = detee(d[0])
        @used = detee(d[1])
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
        return "Pos: #{@pos}, Size: #{@size}, Used: #{@used}"
    end
end

input = File.readlines('input.txt')
    .map { |l| Node.new(l) }