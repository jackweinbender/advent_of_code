require 'digest'

salt = "ihaygndm"
test_salt = "abc"

triples = Hash.new
keys = []

i = 0

while keys.length < 64
    md5 = Digest::MD5.hexdigest "#{salt}#{i.to_s}"
    fiple = md5.downcase.scan(/(.)\1\1\1\1/).flatten
    if not fiple.empty?
        fip = fiple[0]
        if triples.key?(fip)
            keys += triples[fip]
            .select{|key, hash| key + 1000 > i}
            .map{|m| puts "#{m[0]}-#{fip}, #{m[1]}, (#{i}) #{md5}"
                    m}
        end
    end
    triple = md5.scan(/(.)\1\1/).flatten
    if not triple.empty?
        trip = triple[0]
        if triples.key?(trip)
            triples[trip] << [i, md5]
        else
            triples[trip] = [[i, md5]]
        end
    end
    
    i += 1
end
puts "#{keys.max}"
    