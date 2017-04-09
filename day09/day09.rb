

class Parser
    def initialize()
        end
    def parse( str )
        str
    end
    def parsed_len(str)
        return parse( str ).length
    end
end

str = File.readlines('_data.txt')
    .map{ |l| l.chomp }.join('')

p = Parser.new()
puts p.parsed_len(str)

# Starts @ 14248