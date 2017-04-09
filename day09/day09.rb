

class Parser
    def initialize()
    end
    def parse( str )
        m = str.match(/\((\d+)x(\d+)\)/)
        puts "#{m}"
    end
    def parsed_len(str)
        return parse( str ).length
    end
end

str = File.readlines('_data.txt')
    .map{ |l| l.chomp }.join('')

p = Parser.new()
puts p.parse(str)

# Starts @ 14248