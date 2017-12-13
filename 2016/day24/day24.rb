class Astar
    attr_accessor :grid
    def initialize(grid)
        @grid       = Grid.new(grid)
        @visited    = []
    end
    def heuristic(s, e)
        a = (s.x - e.x).abs 
        b = (s.y - e.y).abs

        a + b
    end
    def nexts(node)
        [[0,1],[0,-1],[1,0],[-1,0]]
            .map { |n| [node.x + n[0], node.y + n[1]] }
            .map { |n| "#{n[0]}-#{n[1]}" }
            .select { |n| @grid.hash.has_key?(n) }
            .map { |n| @grid.hash[n] }
            .select { |n| n.type != :wall }
            .reject { |n| @visited.include?(n) }
    end
    def search(start_node, end_node)
        @visited = []
        init = {priority: 0, node: start_node, steps: 0}
        queue = [init]
        
        while queue.length > 0
            
            current = queue.shift
            if current[:node] == end_node 
                return current[:steps]
            end

            nexts = nexts(current[:node])
            nexts.each {|n|
                state = {
                    priority: current[:steps] + heuristic(n, end_node),
                    node: n,
                    steps: current[:steps] + 1
                }
                queue.push(state)
            }
            
            @visited.push(current[:node])
            # puts current[:priority]
            queue.sort! { |a,b| a[:priority] <=> b[:priority] }
        end
        
    end
end

class Grid
    attr_accessor :hash
    def initialize(grid)
        @hash = parse(grid)
    end
    def parse(str)
        grid = Hash.new()
        str.split("\n").each_with_index { |line, y|
            line.split("").each_with_index { |ch, x|
                node = GridNode.new(ch, [x, y])
                hash = node.pos 
                grid[hash] = node
            }
        }
        grid
    end
    def get_waypoints()
        @hash.to_a.select { |n| n[1].type == :waypoint }
    end
end

class GridNode
    attr_accessor :type, :x, :y, :id
    def initialize(ch, pos)
        @type = case ch
        when "#"
            :wall
        when "."
            :hall
        else
            :waypoint
        end
        @x = pos[0]
        @y = pos[1]
        @id = ch.to_i if @type == :waypoint
    end
    def pos
        "#{x}-#{y}"
    end
end

input = File.read("input.txt")
astr = Astar.new(input)

waypoints = astr.grid.hash.to_a
    .select {|n| n[1].type == :waypoint}
    .map {|n| n[1]}

waypoint_distances = waypoints.product(waypoints)
    .map{|pair| pair.sort {|a,b| a.id <=> b.id} }
    .uniq
    .reject {|pair| pair[0].id == pair[1].id}
    .map{|pair|
        path = "#{pair[0].id}-#{pair[1].id}"
        [path, astr.search(pair[0], pair[1])]
    }.to_h
    
answer = waypoints.permutation().to_a
    .select{|n| n[0].id == 0}
    .map {|n| [n, n[0]].flatten }
    .map{ |x| x.map {|y| y.id } }
    .map{|x| 
        l = 0
        x.each_with_index {|n, i|
            if i <= 0
                0
            else
                a = x[i-1]
                b = x[i]
                k = [a,b].sort
                l += waypoint_distances["#{k[0]}-#{k[1]}"]
            end
        }
        l
    }.min

puts "Answer #2\n----"
puts "Shortest path: #{answer}"

