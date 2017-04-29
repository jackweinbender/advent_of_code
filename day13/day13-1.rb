class State
    attr_accessor :steps
    def initialize(state, num, goal, steps)
        @state = state
        @number = num
        @goal = goal
        @steps = steps
    end
    def explode
        combos = []
        combos.push([@state[0], @state[1] + 1])
        combos.push([@state[0] + 1, @state[1]])
        combos.push([@state[0], @state[1] - 1])
        combos.push([@state[0] - 1, @state[1]])

        combos.reject { |x, y| x < 0 || y < 0 }
            .select { |x, y| 
                # puts "x: #{x}, y: #{y}, #{open?(x, y, @number)}"
                open?(x, y, @number)
            }
            .map { |state|
                State.new(state, @number, @goal, @steps + 1)
            }        
    end
    def open?(x, y, n)
        integer = x*x + 3*x + 2*x*y + y + y*y + @number
        open = integer.to_s(2).split('')
            .reject { |i| i == "0" }
            .count % 2
        open == 1 ? false : true
    end
    def complete?
        @state == @goal
    end
    def priority
        steps + hueristic(@state, @goal)
    end
    def hueristic(state, goal)
        (goal[0] - state [0]) + (goal[1] - state[1])
    end
    def to_s
        "#{@state}"
    end
end

init = [1,1]
num = 1358
goal = [31,39]

queue = [] 
visited = []

queue << State.new(init, num, goal, 0)
i = 1
while not queue.empty?
    # i = i+1
    # if  i % 1000 == 0 then
    #     i=1
    #     puts "Steps: #{state.steps} | #{state.to_s}"
    # end
    queue.sort! do |a,b| 
        b.priority <=> a.priority
    end
    state = queue.shift

    if state.complete?
        puts "Completed in #{state.steps} steps."
        exit
    end

    state.explode.each do |new_state|
        next if visited.include?(new_state.to_s)
        # puts "#{new_state.to_s}"
        queue << new_state
        visited << new_state.to_s
    end
end