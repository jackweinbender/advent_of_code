


class Bot 
    def initialize(id, low, high)
        @id = id
        @high = high
        @low = low
        @chips = []
        @match_pair = [17,61]
    end
    def take_chip(chip, graph, out)
        @chips.push(chip.to_i)
        if @chips.sort() == @match_pair then puts "#{@id} is a match." end
        if @chips.length == 2 then
            # puts "Bot #{@id}: #{@chips}"
            dispurse(graph, out)
            @chips = []
        end
    end
    def dispurse(graph, out)
        if @low.match(/^out/) then
            out[@low] = @chips.min
        else
            graph[@low].take_chip(@chips.min, graph, out)
        end
        
        if @high.match(/^out/) then
            out[@high] = @chips.max
        else
            graph[@high].take_chip(@chips.max, graph, out)
        end
    end
end

graph = Hash.new([])
out = Hash.new([])

File.readlines('_data.txt')
    .select { |line| line.match(/^bot (\d+)/) }
    .map { |line| line.match(/(^bot \d+) gives low to (\w+ \d+) and high to (\w+ \d+)/) }
    .each { |m| graph[m[1]] = Bot.new(m[1], m[2], m[3]) }

init = File.readlines('_data.txt')
    .select { |line| line.match(/^value/) }
    .map { |line| line.match(/^value (\d+) goes to (\w+ \d+)/)}
    .each { |m| graph[m[2]].take_chip(m[1], graph, out) }

puts out['output 0'] * out['output 1'] * out['output 2']