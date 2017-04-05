
class IPV
    def initialize( str )
        @hypernets = hypernets_from_string(str)
        @supernets = non_hypernets_from_string(str)
    end
    def hypernets_from_string (str)
        hyps = str.split(/[\[\]]/)
        (0..hyps.length).select { |i| i.odd? }
            .map { |i| hyps[i] }
            .select { |i| i != nil }
    end
    def non_hypernets_from_string (str)
        hyps = str.split(/[\[\]]/)
        (0..hyps.length).select { |i| i.even? }
            .map { |i| hyps[i] }
    end
    def contains_abba?( str )
    str.scan(/([a-zA-Z])([^\1])\2\1/)
        .any? { |m| m != nil && m[0] != m[1] }
    end
    def valid_ipv?
        if @hypernets.any? { |h| contains_abba?(h) }  then
            false
        elsif @supernets.any? { |h| contains_abba?(h) }
            true
        else
            false
        end
    end
    
end

puts input = File.readlines("_data.txt")
    .map { |line| IPV.new(line) }
    .select { |ipv| ipv.valid_ipv? }
    .length
