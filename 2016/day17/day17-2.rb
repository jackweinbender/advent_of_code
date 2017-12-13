require 'digest'

class Cell 
    attr_accessor :path, :pos, :md
    def initialize(pos, path, key)
        @pos = pos
        @path = path
        @key = key
        @md
    end
    def explode
        adjacent = []
        adjacent << Cell.new([@pos[0], @pos[1] - 1], "#{@path}U", @key)
        adjacent << Cell.new([@pos[0], @pos[1] + 1], "#{@path}D", @key)
        adjacent << Cell.new([@pos[0] - 1, @pos[1]], "#{@path}L", @key)
        adjacent << Cell.new([@pos[0] + 1, @pos[1]], "#{@path}R", @key)

        @md = Digest::MD5.hexdigest "#{KEY}#{@path}"
        # puts @md
        @md[0,4].split('')
            .map { |d| ('b'..'f').include? d }
            .zip(adjacent)
            .select { |open, cell| open }
            .reject { |open, cell| cell.pos[0] < 0 || cell.pos[1] < 0 }
            .reject { |open, cell| cell.pos[0] > 3 || cell.pos[1] > 3 }
            .map { |open, cell| cell }
    end
    def at_goal?
        @pos == [3,3]
    end
    def priority
        @path.length + (3 - @pos[0]) + (3 - @pos[1])
    end
end

# KEY = 'ihgpwlah' # Test 1 : DDRRRD
# KEY = 'kglvqrro' # Test 2 : DDUDRLRRUDRD
# KEY = 'ulqzkmiv' # Test 3 : DRURDRUDDLLDLUURRDULRLDUUDDDRR
KEY = 'qtetzkpl'

paths = []
max = 0
queue = []
queue << Cell.new([0,0], "", KEY)
i = 1
while not queue.empty?
    i += 1
    queue.sort! do |a,b| 
        b.priority <=> a.priority
    end

    state = queue.shift
    # puts "Current path: #{state.path} \n #{state.md5}"
    if state.at_goal?
        if state.path.length > max
            max = state.path.length
        end
        puts "Completed by path: #{state.path}"
        next
    end

    state.explode.each do |new_state|
        # puts new_state.path
        queue << new_state
    end
end 

puts "Longest path: #{max}"