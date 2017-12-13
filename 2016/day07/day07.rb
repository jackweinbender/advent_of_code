# http://adventofcode.com/2016/day/7
require 'json'

class IPV
    def initialize( str )
        @hypernets = hypernets_from_string(str)
        @supernets = supernets_from_string(str)
    end
    def hypernets_from_string (str)
        hyps = str.split(/[\[\]]/)
        (0..hyps.length).select { |i| i.odd? }
            .map { |i| hyps[i] }
            .select { |i| i != nil }
    end
    def supernets_from_string (str)
        sups = str.split(/[\[\]]/)
        (0..sups.length).select { |i| i.even? }
            .map { |i| sups[i] }
            .select { |i| i != nil }
    end
    def contains_abba?( str )
    str.scan(/(\w)(\w)\2\1/)
        .any? { |m| m != nil && m[-1] != m[-2] }
    end
    def get_abas( arr )
        arr.map { |str| str
            out = []
            max = str.length - 2;
            for i in 0..max
                if str[i] == str[i + 2] && str[i] != str[i+1] then
                    # puts "#{str[i]}#{str[i+1]}#{str[i]}"
                    out.push("#{str[i]}#{str[i+1]}#{str[i]}")
                end
            end
            out
        }.flatten()
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
    def valid_ssl?
        abas = get_abas(@supernets)
        babs = get_abas(@hypernets)
            .map {|aba| "#{aba[1]}#{aba[0]}#{aba[1]}" }
        abas.any? { |aba| babs.include? aba }
    end
end

# # Answer #1
ans1 = File.readlines("_data.txt")
    .map { |line| IPV.new(line) }
    .select { |ipv| ipv.valid_ipv? }
    .length
puts "Answer 1: #{ans1}"

# # Answer #2

ans2 = File.readlines("_data.txt")
    .map { |line| IPV.new(line) }
    .select { |ipv| ipv.valid_ssl? }
    .length

puts "Answer 2: #{ans2}"