
class State
    def initialize(state, step)
        @generators = state[:generators].map(&:sort)
        @microchips = state[:microchips].map(&:sort)
        @elevator = state[:elevator]
        @step = step
    end
    def steps 
        @step
    end
    def complete?
        @generators[0..2].flatten.empty? && @microchips[0..2].flatten.empty?
    end

    def to_s
        "gens: #{@generators}, chips: #{@microchips}, el: #{@elevator}"
    end
    def valid?
        (0..3).all? { |floor| floor_valid?(floor) }
    end
    def floor_valid?(floor)
        unpowered_chips = @microchips[floor] - @generators[floor]
        unpowered_chips.none? || @generators[floor].count == 0
    end
    def explode
        
        floors = [1,-1].map { |d| @elevator + d }
            .reject{ |f| f < 0 || f > 3 }
        
        movables = []

        movables += @generators[@elevator]
            .product(@microchips[@elevator])
            .reject { |g,m| g != m }
            .map { |g,m| {gens: [g], chips: [m]} }
        (1..2).each do |take| 
            movables += @generators[@elevator]
                .combination(take)
                .map{ |g| {gens: g.compact, chips: []} }
        end
        (1..2).each do |take| 
            movables += @microchips[@elevator]
                .combination(take)
                .map{ |m| {gens: [], chips: m.compact} }
        end
        
        states = movables.product(floors).map do |combo, floor|
            state = {
                generators: @generators.map(&:clone),
                microchips: @microchips.map(&:clone),
                elevator: floor
            }
            combo[:gens].each do |gen|
                state[:generators][floor] << state[:generators][@elevator].delete(gen)
            end
            combo[:chips].each do |chip|
                state[:microchips][floor] << state[:microchips][@elevator].delete(chip)
            end
            
            State.new(state, @step + 1)
        end

        states.flatten.select(&:valid?)

    end
end

# Part 1
state = {
    generators:[["pr"], ["co", "cu", "ru", "pl"], [], []],
    microchips:[["pr"], [], ["co", "cu", "ru", "pl"], []],
    elevator: 0
}

#Part 2
state_2 = {
    generators:[["pr"], ["co", "cu", "ru", "pl"], [], []],
    microchips:[["pr"], [], ["co", "cu", "ru", "pl"], []],
    elevator: 0
}

queue = Queue.new 
visited = []

queue << State.new(state, 0)
i = 1
while not queue.empty?
    i = i+1
    if  i % 1000 == 0 then
        i=1
        puts "Steps: #{state.steps} | #{state.to_s}"
    end
    state = queue.shift

    if state.complete?
        puts "Completed in #{state.steps} steps."
        exit
    end

    state.explode.each do |new_state|
        next if visited.include?(new_state.to_s)
        queue << new_state
        visited.push(new_state.to_s)
    end
end