
class IPV
    def initialize( str )
        @hypernets = hypernets_from_string(str)
        @non_hypernets = non_hypernets_from_string(str)
        @valid_ipv = valid_ipv(@hypernets, @non_hypernets)
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
    def valid_ipv( hypernets_arr, non_hypernets_arr )
        if hypernets_arr.any? { |h| contains_abba?(h) }  then
            false
        elsif non_hypernets_arr.any? { |h| contains_abba?(h) }
            true
        else
            false
        end
    end
    def valid?
        @valid_ipv
    end
    
end

puts input = File.readlines("_data.txt")
    .map { |line| IPV.new(line) }
    .select { |ipv| ipv.valid? }
    .length
