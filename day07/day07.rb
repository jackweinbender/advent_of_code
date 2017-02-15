
class IPV
    def initialize( str )
        @head, @brackets, @tail = from_string(str)
        @valid_ipv = valid_ipv(from_string(str))
    end
    def from_string (str)
        str.split(/[\[\]]/)
    end
    def contains_abba?( str )
    str.scan(/([a-zA-Z])([^\1])\2\1/)
        .any? { |m| m != nil && m[0] != m[1] }
    end
    def valid_ipv( arr )
        if contains_abba?(arr[1]) then
            false
        elsif contains_abba?(arr[0]) || contains_abba?(arr[2])
            true
        else
            false
        end
    end
    def valid?
        @valid_ipv
    end
    def to_string
        puts "#{@head}[#{@brackets}]#{@tail}"
    end
    
end

puts input = File.readlines("_test_data.txt")
    .map { |line| IPV.new(line) }
    .select { |ipv| ipv.valid? }
    .map { |ipv| ipv.to_string() }
    .length
