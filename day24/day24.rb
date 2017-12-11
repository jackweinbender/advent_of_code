class Astar
    attr_accessor :grid
    def initialize(grid)
        @grid       = Grid.new(grid)
        @visited    = []
    end
    def heuristic(s, e)
        (s.x - e.x).abs + (s.y - e.y).abs
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
            queue.sort! { |a,b| a[1] <=> b[1] }
        end
        
    end
end

class Node
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

class Grid
    attr_accessor :hash
    def initialize(grid)
        @hash = parse(grid)
    end
    def parse(str)
        grid = Hash.new()
        str.split("\n").each_with_index { |line, y|
            line.split("").each_with_index { |ch, x|
                node = Node.new(ch, [x, y])
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

test = File.read("test_input.txt")

astr = Astar.new(test)

start_node  = astr.grid.hash["1-1"]
end_node    = astr.grid.hash["1-3"]
puts astr.search(start_node, end_node)

